use std::collections::HashMap;

use crate::data::ReadRecord;
use crate::data::WriteRecord;
use crate::error::Result;

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
    fn region_wcus(&self) -> Result<HashMap<RegionId, WcuCount>>;

    /// Get all rcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn region_rcus(&self) -> Result<HashMap<RegionId, WcuCount>>;

    /// Get all wcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn table_wcus(&self) -> Result<HashMap<TableId, WcuCount>>;

    /// Get all rcu data by region dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn table_rcus(&self) -> Result<HashMap<TableId, WcuCount>>;

    /// Get all wcu data by schema dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn schema_wcus(&self) -> Result<HashMap<SchemaId, WcuCount>>;

    /// Get all rcu data by schema dimension.
    ///
    /// Note: If clear is executed, the previous data will not be counted.
    fn schema_rcus(&self) -> Result<HashMap<SchemaId, RcuCount>>;

    /// Clear all data.
    fn clear(&self);
}

/// The SchemaId identifies a database.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct SchemaId {
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
