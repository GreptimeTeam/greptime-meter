// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "noop")]
#[macro_export]
macro_rules! write_meter {
    ($catalog: expr, $schema: expr, $write_calc: expr) => {
        let _ = ($catalog, $schema, &$write_calc);
    };
    ($catalog: expr, $schema: expr, $table: expr, $write_calc: expr) => {
        let _ = ($catalog, $schema, $table, &$write_calc);
    };
    ($catalog: expr, $schema: expr, $table: expr, $region: expr, $write_calc: expr) => {
        let _ = ($catalog, $schema, $table, $region, &$write_calc);
    };
}

/// Record some about data insertion.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
///
/// use meter_core::write_calc::WriteCalculator;
/// use meter_core::global::global_registry;
/// use meter_macros::write_meter;
///
/// // A struct about insert request
/// struct MockInsert;
///
/// // A byte count calculator of insert request
/// struct MockInsertCalculator;
///
/// impl WriteCalculator<MockInsert> for MockInsertCalculator {
///     fn calc_byte(&self, _: &MockInsert) -> u32 {
///        10 * 1024
///     }
/// }
///
/// let calculator = MockInsertCalculator;
///
/// // Register a calculator to [registry].
/// let registry = global_registry();
/// registry.register_calculator(Arc::new(MockInsertCalculator));
///
/// write_meter!("greptime", "public", MockInsert);
/// ```

#[cfg(not(feature = "noop"))]
#[macro_export]
macro_rules! write_meter {
    ($catalog: expr, $schema: expr, $write_calc: expr) => {
        let r = meter_core::global::global_registry();

        if let Some(calc) = r.get_calculator() {
            let byte_count = calc.calc_byte(&$write_calc);

            let record = meter_core::data::WriteRecord {
                catalog: $catalog.to_string(),
                schema: $schema.to_string(),
                table: None,
                region_num: None,
                byte_count,
            };

            r.record_write(record);
        };
    };

    ($catalog: expr, $schema: expr, $table: expr, $write_calc: expr) => {
        let r = meter_core::global::global_registry();

        if let Some(calc) = r.get_calculator() {
            let byte_count = calc.calc_byte(&$write_calc);

            let record = meter_core::data::WriteRecord {
                catalog: $catalog.to_string(),
                schema: $schema.to_string(),
                table: Some($table.to_string()),
                region_num: None,
                byte_count,
            };

            r.record_write(record);
        };
    };

    ($catalog: expr, $schema: expr, $table: expr, $region: expr, $write_calc: expr) => {
        let r = meter_core::global::global_registry();

        if let Some(calc) = r.get_calculator() {
            let byte_count = calc.calc_byte(&$write_calc);

            let record = meter_core::data::WriteRecord {
                catalog: $catalog.to_string(),
                schema: $schema.to_string(),
                table: Some($table.to_string()),
                region_num: Some($region),
                byte_count,
            };

            r.record_write(record);
        };
    };
}
