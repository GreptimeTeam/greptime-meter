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
macro_rules! write_meter {
    ($catalog: expr, $schema: expr, $write_calc: expr, $source: expr) => {{
        let _ = ($catalog, $schema, &$write_calc, $source);
        0 as u64
    }};
}

/// Record some about data insertion.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
///
/// use meter_core::ItemCalculator;
/// use meter_core::global::global_registry;
/// use meter_core::data::TrafficSource;
/// use meter_macros::write_meter;
///
/// // A struct about insert request
/// struct MockInsert;
///
/// // A byte count calculator of insert request
/// struct MockInsertCalculator;
///
/// impl ItemCalculator<MockInsert> for MockInsertCalculator {
///     fn calc(&self, _: &MockInsert) -> u64 {
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
/// write_meter!("greptime", "public", MockInsert, 0);
/// ```
#[cfg(not(feature = "noop"))]
#[macro_export]
macro_rules! write_meter {
    ($catalog: expr, $schema: expr, $req_item: expr, $source: expr) => {{
        let r = meter_core::global::global_registry();
        let mut value = 0;
        if let Some(calc) = r.get_calculator() {
            value = calc.calc(&$req_item);

            let record = meter_core::data::MeterRecord {
                catalog: $catalog.into(),
                schema: $schema.into(),
                value: value,
                source: source,
            };

            r.record_write(record);
        };
        value
    }};
}
