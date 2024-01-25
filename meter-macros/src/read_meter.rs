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
    ($catalog: expr, $schema: expr, cpu_time: $cpu_time: expr) => {
        let _ = ($catalog, $schema, $cpu_time);
    };
    ($catalog: expr, $schema: expr, table_scan: $table_scan: expr) => {
        let _ = ($catalog, $schema, $table_scan);
    };
    ($catalog: expr, $schema: expr, $cpu_time: expr, $table_scan: expr) => {
        let _ = ($catalog, $schema, $cpu_time, $table_scan);
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
///
/// read_meter!("greptime", "public", cpu_time: cpu_time_ns);
/// read_meter!("greptime", "public", table_scan: table_scan_bytes);
///
/// read_meter!(
///     "greptime",
///     "public",
///     cpu_time_ns,
///     table_scan_bytes
/// );
/// ```
#[cfg(not(feature = "noop"))]
#[macro_export]
macro_rules! read_meter {
    ($catalog: expr, $schema: expr, cpu_time: $cpu_time: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            cpu_time: $cpu_time,
            table_scan: 0,
        };
        meter_core::global::global_registry().record_read(record);
    };
    ($catalog: expr, $schema: expr, table_scan: $table_scan: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            cpu_time: 0,
            table_scan: $table_scan,
        };
        meter_core::global::global_registry().record_read(record);
    };
    ($catalog: expr, $schema: expr, $cpu_time: expr, $table_scan: expr) => {
        let record = meter_core::data::ReadRecord {
            catalog: $catalog.into(),
            schema: $schema.into(),
            cpu_time: $cpu_time,
            table_scan: $table_scan,
        };
        meter_core::global::global_registry().record_read(record);
    };
}
