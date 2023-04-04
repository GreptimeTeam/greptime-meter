/// The WriteRecord records some data that consumes wcu.
#[derive(Debug)]
pub struct WriteRecord {
    pub catalog: String,
    pub schema: String,
    pub table: Option<String>,
    pub region_num: Option<u32>,

    pub byte_count: u32,

    /// Use Scenarios of the record.
    ///
    /// Collector can procces the record according to the usage scenarios.
    pub scenes: Scenes,
}

/// The ReadRecord records some data that consumes rcu.
#[derive(Debug)]
pub struct ReadRecord {
    pub catalog: String,
    pub schema: String,
    pub table: Option<String>,
    pub region_num: Option<u32>,

    // TODO(fys): is millisecond accurate enough?
    /// Unit is millisecond.
    pub cpu_time: u32,
    /// Unit is byte.
    pub table_scan: u32,
    /// Unit is byte.
    pub network_egress: u32,

    /// Use Scenarios of the record.
    ///
    /// Collector can procces the record according to the usage scenarios.
    pub scenes: Scenes,
}

#[derive(Debug)]
pub enum Scenes {
    RateLimit,
    DistributedScheduling,
}

/// The ServiceId identifies a specific database.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ServiceId {
    pub catalog: String,
    pub schema: String,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct TableId {
    pub catalog: String,
    pub schema: String,
    pub table: String,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct RegionId {
    pub catalog: String,
    pub schema: String,
    pub table: String,
    pub region_num: u32,
}
