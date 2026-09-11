// Copyright 2024 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use parking_lot::RwLock;
use tracing::warn;

use crate::collect::Collect;
use crate::collect::WriteRejected;
use crate::data::MeterRecord;
use crate::ItemCalculator;

type CalculatorMap = anymap2::SendSyncAnyMap;

#[derive(Default, Clone)]
pub struct Registry {
    inner: Arc<Inner>,
}

struct Inner {
    collector: RwLock<Option<Arc<dyn Collect>>>,
    calculator: RwLock<CalculatorMap>,
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            collector: Default::default(),
            calculator: RwLock::new(CalculatorMap::new()),
        }
    }
}

impl Registry {
    /// Set [Collect] for [Registry].
    pub fn set_collector(&self, collector: Arc<dyn Collect>) {
        let mut guard = self.inner.collector.write();
        *guard = Some(collector);
    }

    /// Register the calculation formula of 'insert request' -> 'byte count'
    pub fn register_calculator<T: Send + Sync + 'static>(
        &self,
        calculator: Arc<dyn ItemCalculator<T>>,
    ) {
        let mut guard = self.inner.calculator.write();
        guard.insert(calculator);
    }

    /// Obtain the calculation formula corresponding to the insert request.
    pub fn get_calculator<T: Send + Sync + 'static>(&self) -> Option<Arc<dyn ItemCalculator<T>>> {
        let guard = self.inner.calculator.read();
        if let Some(calc) = (*guard).get::<Arc<dyn ItemCalculator<T>>>().cloned() {
            Some(calc)
        } else {
            warn!(
                "[meter]cannot find calculator for type: {:?}",
                std::any::type_name::<T>()
            );
            None
        }
    }
}

impl Registry {
    /// Admits and records a write, or succeeds if no collector is registered.
    ///
    /// Collection runs without holding the registry lock. The collector owns
    /// admission policy and must reject before recording accepted usage.
    /// Success does not imply persistence, and the registry performs no rollback.
    pub async fn record_write(&self, record: MeterRecord) -> Result<(), WriteRejected> {
        let collector = self.inner.collector.read().clone();
        if let Some(collector) = collector {
            collector.on_write(record).await?;
        }
        Ok(())
    }

    /// A base API for recording information about data query.
    pub fn record_read(&self, record: MeterRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(c) => c,
            None => return,
        };

        collector.on_read(record);
    }
}

#[cfg(test)]
mod tests {
    use std::future::poll_fn;
    use std::future::Future;
    use std::pin::pin;
    use std::pin::Pin;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Waker;

    use super::*;

    struct SuspendedCollector {
        resume: AtomicBool,
    }

    impl Collect for SuspendedCollector {
        fn on_write(
            &self,
            record: MeterRecord,
        ) -> Pin<Box<dyn Future<Output = Result<(), WriteRejected>> + Send + '_>> {
            Box::pin(async move {
                poll_fn(|_| {
                    if self.resume.load(Ordering::Relaxed) {
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                })
                .await;
                assert_eq!(record.rows, u64::MAX);
                Err(WriteRejected::new("quota exhausted"))
            })
        }

        fn on_read(&self, _: MeterRecord) {}
    }

    #[test]
    fn write_collection_releases_lock_and_propagates_rejection() {
        let registry = Registry::default();
        let record = || MeterRecord::new("catalog".into(), "schema".into(), 0, u64::MAX, 7);
        let mut cx = Context::from_waker(Waker::noop());
        assert!(matches!(
            pin!(registry.record_write(record())).poll(&mut cx),
            Poll::Ready(Ok(()))
        ));

        let collector = Arc::new(SuspendedCollector {
            resume: AtomicBool::new(false),
        });
        registry.set_collector(collector.clone());
        let future = registry.record_write(record());
        fn assert_send(_: &impl Send) {}
        assert_send(&future);
        let mut future = pin!(future);
        assert!(future.as_mut().poll(&mut cx).is_pending());

        // Changing registration while collection is suspended must not block.
        *registry
            .inner
            .collector
            .try_write()
            .expect("collector lock retained") = None;
        collector.resume.store(true, Ordering::Relaxed);
        let Poll::Ready(Err(error)) = future.as_mut().poll(&mut cx) else {
            panic!("expected write rejection");
        };
        let error: &dyn std::error::Error = &error;
        assert_eq!(error.to_string(), "quota exhausted");
    }
}
