use crate::data::ReadRecord;
use crate::data::WriteRecord;

/// Trait representing the methods required to collect read/write record.
pub trait Collect: Send + Sync {
    /// Notifies the method that an event that consumes wcu occurs.
    fn on_read(&self, record: ReadRecord);

    /// Notifies the method that an event that consumes rcu occurs.
    fn on_write(&self, record: WriteRecord);
}
