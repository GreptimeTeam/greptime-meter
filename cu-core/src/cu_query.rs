use std::collections::HashMap;

use crate::data::ReadRecord;
use crate::data::WriteRecord;

pub type WcuCount = u32;
pub type RcuCount = u32;

/// Trait representing the methods of querying cu data.
pub trait CuQuery<W, R>: Send + Sync
where
    W: Fn(&WriteRecord) -> WcuCount,
    R: Fn(&ReadRecord) -> RcuCount,
{
    /// Set the calculation formula of "WriteRecord -> wcus".
    fn set_wcu_calc(&mut self, calc: W);

    /// Set the calculation formula of "ReadRecord -> rcus".
    fn set_rcu_calc(&mut self, calc: R);

    /// Get all wcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn region_wcus(&self) -> HashMap<RegionId, WcuCount>;

    /// Get all rcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn region_rcus(&self) -> HashMap<RegionId, WcuCount>;

    /// Get all wcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn table_wcus(&self) -> HashMap<TableId, WcuCount>;

    /// Get all rcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn table_rcus(&self) -> HashMap<TableId, WcuCount>;

    /// Get all wcu data by service dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn service_wcus(&self) -> HashMap<ServiceId, WcuCount>;

    /// Get all rcu data by service dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn service_rcus(&self) -> HashMap<ServiceId, RcuCount>;

    /// Clear all data.
    fn clear(&self);
}

/// The ServiceId identifies a database.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ServiceId {
    pub catalog: String,
    pub schema: String,
}

/// The TableId identifies a table.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct TableId {
    pub catalog: String,
    pub schema: String,
    pub table: String,
}

/// The RegionId identifies a region.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct RegionId {
    pub catalog: String,
    pub schema: String,
    pub table: String,
    pub region_num: u32,
}
