use std::sync::Arc;

use parking_lot::RwLock;

use crate::collect::Collect;
use crate::data::ReadRecord;
use crate::data::WriteRecord;
use crate::reporter::Reporter;

#[derive(Default, Clone)]
pub struct Registry {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    collector: RwLock<Option<Arc<dyn Collect>>>,
    reporter: RwLock<Option<Arc<dyn Reporter>>>,
}

impl Registry {
    pub fn set_collector(&self, collector: Arc<dyn Collect>) {
        let mut guard = self.inner.collector.write();
        *guard = Some(collector);
    }

    pub fn set_reporter(&mut self, reporter: Arc<dyn Reporter>) {
        let mut guard = self.inner.reporter.write();
        *guard = Some(reporter);
    }
}

impl Registry {
    pub async fn start(&self) {
        let reporter_clone = {
            let reporter = self.inner.reporter.read();
            reporter.clone()
        };

        if let Some(reporter) = reporter_clone.as_ref() {
            reporter.start().await;
        }
    }

    pub fn record_write(&self, info: WriteRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(collector) => collector,
            None => return,
        };

        collector.on_write(info);
    }

    pub fn record_read(&self, info: ReadRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(c) => c,
            None => return,
        };

        collector.on_read(info);
    }

    // TODO(fys): provide better api design
}
