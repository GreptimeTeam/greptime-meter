// Copyright 2024 Greptime Team
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
macro_rules! read_meter {
    ($catalog: expr, $schema: expr, $item: expr, $source: expr) => {{
        let _ = ($catalog, $schema, $item, $source);
        0 as u64
    }};
}

/// Record some about data query.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
///
/// use meter_core::ItemCalculator;
/// use meter_core::global::global_registry;
/// use meter_macros::read_meter;
/// use meter_core::data::ReadItem;
///
/// let cpu_time_ns = 1000000000;
/// let table_scan_bytes = 10224378;
///
/// // A struct about insert request
/// struct MockInsert;
///
/// // A byte count calculator of insert request
/// struct MockInsertCalculator;
///
/// impl ItemCalculator<ReadItem> for MockInsertCalculator {
///     fn calc(&self, _: &ReadItem) -> u64 {
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
/// read_meter!("greptime", "public", ReadItem {
///     cpu_time: cpu_time_ns,
///     table_scan: table_scan_bytes,
/// }, 0);
/// ```
#[cfg(not(feature = "noop"))]
#[macro_export]
macro_rules! read_meter {
    ($catalog: expr, $schema: expr, $item: expr, $source: expr) => {{
        let r = meter_core::global::global_registry();
        let mut value = 0;
        if let Some(calc) = r.get_calculator() {
            value = calc.calc(&$item);
            let record = meter_core::data::MeterRecord {
                catalog: $catalog.into(),
                schema: $schema.into(),
                value: value,
                source: $source,
            };
            meter_core::global::global_registry().record_read(record);
        }
        value
    }};
}
