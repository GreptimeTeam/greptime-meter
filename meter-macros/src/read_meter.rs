#[cfg(feature = "noop")]
#[macro_export]
macro_rules! read_meter {
    ($catalog: expr, $schema: expr, cpu_time: $cpu_time: expr) => {
        let _ = ($catalog, $schema, $cpu_time);
    };
    ($catalog: expr, $schema: expr, table_scan: $table_scan: expr) => {
        let _ = ($catalog, $schema, $schema);
    };
    ($catalog: expr, $schema: expr, network_egress: $network_egress: expr) => {
        let _ = ($catalog, $schema, $network_egress);
    };
    ($catalog: expr, $schema: expr, $cpu_time: expr, $table_scan: expr, $network_egress: expr) => {
        let _ = ($catalog, $schema, $cpu_time, $table_scan, $network_egress);
    };
}

/// Record some about data query.
///
/// # Examples
///
/// ```rust
/// use meter_macros::read_meter;
///
/// let cpu_time_ns = 1000000000;
/// let table_scan_bytes = 10224378;
/// let network_egress_bytes = 1023123;
///
/// read_meter!("greptime", "public", cpu_time: cpu_time_ns);
/// read_meter!("greptime", "public", table_scan: table_scan_bytes);
/// read_meter!("greptime", "public", network_egress: network_egress_bytes);
///
/// read_meter!(
///     "greptime",
///     "public",
///     cpu_time_ns,
///     table_scan_bytes,
///     network_egress_bytes
/// );
/// ```
#[cfg(not(feature = "noop"))]
#[macro_export]
macro_rules! read_meter {
    ($catalog: expr, $schema: expr, cpu_time: $cpu_time: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            table: None,
            region_num: None,
            cpu_time: $cpu_time,
            table_scan: 0,
            network_egress: 0,
        };
        meter_core::global::global_registry().record_read(record);
    };
    ($catalog: expr, $schema: expr, table_scan: $table_scan: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            table: None,
            region_num: None,
            cpu_time: 0,
            table_scan: $table_scan,
            network_egress: 0,
        };
        meter_core::global::global_registry().record_read(record);
    };
    ($catalog: expr, $schema: expr, network_egress: $network_egress: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            table: None,
            region_num: None,
            cpu_time: 0,
            table_scan: 0,
            network_egress: $network_egress,
        };
        meter_core::global::global_registry().record_read(record);
    };
    ($catalog: expr, $schema: expr, $cpu_time: expr, $table_scan: expr, $network_egress: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            table: None,
            region_num: None,
            cpu_time: $cpu_time,
            table_scan: $table_scan,
            network_egress: $network_egress,
        };
        meter_core::global::global_registry().record_read(record);
    };
}
