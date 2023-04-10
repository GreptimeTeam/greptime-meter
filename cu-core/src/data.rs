/// The WriteRecord records some data about consumed wcu.
#[derive(Debug)]
pub struct WriteRecord {
    pub catalog: String,
    pub schema: String,
    pub table: Option<String>,
    pub region_num: Option<u32>,

    /// Volume of data written in byte.
    pub byte_count: u32,

    /// Use Scenarios of the record.
    ///
    /// Collector can procces the record according to the usage scenarios.
    pub scenes: Scenes,
}

/// The ReadRecord records some data about consumed rcu.
#[derive(Debug)]
pub struct ReadRecord {
    pub catalog: String,
    pub schema: String,
    pub table: Option<String>,
    pub region_num: Option<u32>,

    // TODO(fys): is millisecond accurate enough?
    /// The CPU consumed by query SQL processes.
    ///
    /// Unit is millisecond.
    pub cpu_time: u32,

    /// The data size of table scan plan.
    ///
    /// Unit is byte.
    pub table_scan: u32,

    /// The size of the network traffic used by the query.
    ///
    /// Unit is byte.
    pub network_egress: u32,

    /// Use Scenarios of the record.
    ///
    /// Collector can procces the record according to the usage scenarios.
    pub scenes: Scenes,
}

/// Use Scenarios of the record.
#[derive(Debug)]
pub enum Scenes {
    RateLimit,
    DistributedScheduling,
}
