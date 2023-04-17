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

/// Record the consumption of wcu.
///
/// # Examples
///
/// ```rust
/// use cu_core::write_calc::WriteCalculator;
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
/// // calculate byte count of a custom type
/// struct MockInsert;
/// // 1. use `From` to calculate the byte count.
/// impl From<&MockInsert> for u32 {
///     fn from(_: &MockInsert) -> Self {
///         10 * 1024
///     }
/// }
///
/// wcu!("greptime", "public", (&MockInsert).into());
///
/// // 2. use a calculator
/// struct MockInsertCalculator;
/// impl WriteCalculator<MockInsert> for MockInsertCalculator {
///     fn calc_byte(&self, _value: &MockInsert) -> u32 {
///        10 * 1024
///     }
/// }
/// let calculator = MockInsertCalculator;
/// wcu!("greptime", "public", calculator.calc_byte(&MockInsert));
/// ```
#[macro_export]
macro_rules! wcu {
    ($catalog: expr, $schema: expr, $write_calc: expr) => {
        let record = cu_core::data::WriteRecord {
            catalog: $catalog.to_string(),
            schema: $schema.to_string(),
            table: None,
            region_num: None,
            byte_count: $write_calc,
        };
        cu_core::global::global_registry().record_write(record);
    };
    ($catalog: expr, $schema: expr, $table: expr, $write_calc: expr) => {
        let record = cu_core::data::WriteRecord {
            catalog: $catalog.to_string(),
            schema: $schema.to_string(),
            table: Some($table.to_string()),
            region_num: None,
            byte_count: $write_calc,
        };
        cu_core::global::global_registry().record_write(record);
    };
    ($catalog: expr, $schema: expr, $table: expr, $region: expr, $write_calc: expr) => {
        let record = cu_core::data::WriteRecord {
            catalog: $catalog.to_string(),
            schema: $schema.to_string(),
            table: Some($table.to_string()),
            region_num: Some($region),
            byte_count: $write_calc,
        };
        cu_core::global::global_registry().record_write(record);
    };
}
