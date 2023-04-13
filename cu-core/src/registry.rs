use std::sync::Arc;

use parking_lot::RwLock;

use crate::collect::Collect;
use crate::data::ReadRecord;
use crate::data::WriteRecord;

#[derive(Default, Clone)]
pub struct Registry {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    collector: RwLock<Option<Arc<dyn Collect>>>,
}

impl Registry {
    /// Set [Collect] for [Registry].
    pub fn set_collector(&self, collector: Arc<dyn Collect>) {
        let mut guard = self.inner.collector.write();
        *guard = Some(collector);
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
