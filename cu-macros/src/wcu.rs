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

#[cfg(test)]
mod tests {
    use cu_core::write_calc::WriteCalc;

    #[test]
    fn test_wcu_macro() {
        let catalog = "greptime";
        let schema = "public";
        let table = "system_log";
        let region_num = 1;
        let byte_count = 1024;

        wcu!(catalog, schema, table, region_num, byte_count);
        wcu!("catalog".to_string(), schema, table, byte_count);
        wcu!(catalog, schema, byte_count);

        struct MockInsert;

        impl WriteCalc for MockInsert {
            fn byte_count(&self) -> u32 {
                10 * 1024
            }
        }
        wcu!(catalog, schema, MockInsert);
        wcu!(catalog, schema, MockInsert.byte_count());
        wcu!(catalog, schema, table, region_num, MockInsert);
        wcu!("catalog".to_string(), schema, table, MockInsert);
    }
}
