/// Record the consumption of wcu.
///
/// # Examples
///
/// ```rust
/// use cu_core::write_calc::WriteCalc;
/// use cu_macros::wcu;
///
/// // catalog: "greptime", schema: "public", byte_count: 1024 * 10.
/// wcu!("greptime", "public", 1024 * 10);
///
/// // catalog: "greptime", schema: "public", table: "system_log", byte_count: 1024 * 10.
/// wcu!("greptime", "public", "system_log", 1024 * 10);
///
/// // catalog: "greptime", schema: "public", table: "system_log", region: 0, byte_count: 1024 * 10.
/// wcu!("greptime", "public", "system_log", 0, 1024 * 10);
///
/// struct MockInsert;
///
/// impl WriteCalc for MockInsert {
///     fn byte_count(&self) -> u32 {
///         10 * 1024
///     }
/// }
///
/// wcu!("greptime", "public", MockInsert);
/// ```
#[macro_export]
macro_rules! wcu {
    ($catalog: expr, $schema: expr, $write_calc: expr) => {
        let record = cu_core::data::WriteRecord {
            catalog: $catalog.to_string(),
            schema: $schema.to_string(),
            table: None,
            region_num: None,
            byte_count: cu_core::write_calc::WriteCalc::byte_count(&$write_calc),
        };
        cu_core::global::global_registry().record_write(record);
    };
    ($catalog: expr, $schema: expr, $table: expr, $write_calc: expr) => {
        let record = cu_core::data::WriteRecord {
            catalog: $catalog.to_string(),
            schema: $schema.to_string(),
            table: Some($table.to_string()),
            region_num: None,
            byte_count: cu_core::write_calc::WriteCalc::byte_count(&$write_calc),
        };
        cu_core::global::global_registry().record_write(record);
    };
    ($catalog: expr, $schema: expr, $table: expr, $region: expr, $write_calc: expr) => {
        let record = cu_core::data::WriteRecord {
            catalog: $catalog.to_string(),
            schema: $schema.to_string(),
            table: Some($table.to_string()),
            region_num: Some($region),
            byte_count: cu_core::write_calc::WriteCalc::byte_count(&$write_calc),
        };
        cu_core::global::global_registry().record_write(record);
    };
}
