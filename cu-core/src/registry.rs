// Copyright 2023 Greptime Team
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

use anymap::any::Any;
use parking_lot::RwLock;

use crate::collect::Collect;
use crate::data::ReadRecord;
use crate::data::WriteRecord;
use crate::write_calc::WriteCalculator;

type CalculatorMap = anymap::Map<dyn Any + Send + Sync>;

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

    /// The calculation formula of 'insert request' -> 'byte count'
    pub fn register_calculator<T: Send + Sync + 'static>(
        &self,
        calculator: Arc<dyn WriteCalculator<T>>,
    ) {
        let mut guard = self.inner.calculator.write();
        guard.insert(calculator);
    }

    /// Obtain the calculation formula corresponding to the insert request.
    pub fn get_calculator<T: Send + Sync + 'static>(&self) -> Option<Arc<dyn WriteCalculator<T>>> {
        let guard = self.inner.calculator.read();
        (*guard).get::<Arc<dyn WriteCalculator<T>>>().cloned()
    }
}

impl Registry {
    /// A base API for recording WCU consumption.
    pub fn record_write(&self, record: WriteRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(collector) => collector,
            None => return,
        };

        collector.on_write(record);
    }

    /// A base API for recording RCU consumption.
    pub fn record_read(&self, record: ReadRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(c) => c,
            None => return,
        };

        collector.on_read(record);
    }
}
