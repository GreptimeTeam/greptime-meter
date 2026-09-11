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

//! Admit and record writes before dispatch. Successful collection does not imply
//! successful persistence. Rejection must be handled before dispatching the write.
//!
//! ```rust
//! use std::sync::Arc;
//! use meter_core::{ItemCalculator, collect::WriteRejected, data::MeterRecord};
//! use meter_core::global::global_registry;
//! use meter_macros::write_meter;
//!
//! struct InsertRequest;
//! struct Calculator;
//! impl ItemCalculator<InsertRequest> for Calculator {
//!     fn calc(&self, _: &InsertRequest) -> u64 { 1024 }
//! }
//!
//! # async fn example() -> Result<(), WriteRejected> {
//! global_registry().register_calculator(Arc::new(Calculator));
//! let request = InsertRequest;
//! let cost = write_meter!("greptime", "public", request, 10, 0).await?;
//! // Dispatch `request` only after admission succeeds.
//!
//! // Bulk writes can submit a precomputed value without a calculator.
//! let cost = write_meter!(MeterRecord::new(
//!     "greptime".into(), "public".into(), 0, 10, 0,
//! )).await?;
//! # Ok(())
//! # }
//! ```

/// Returns `Ok(0)` without evaluating arguments or invoking collection.
#[cfg(feature = "noop")]
#[macro_export]
macro_rules! write_meter {
    ($catalog:expr, $schema:expr, $req_item:expr, $rows:expr, $source:expr) => {{
        // Type-check and mark arguments as used without scanning rows or moving requests.
        if false {
            let _ = (&$catalog, &$schema, &$req_item, &$rows, &$source);
        }
        std::future::ready(Ok::<u64, meter_core::collect::WriteRejected>(0))
    }};
    ($record:expr) => {{
        if false {
            let _: &meter_core::data::MeterRecord = &$record;
        }
        std::future::ready(Ok::<u64, meter_core::collect::WriteRejected>(0))
    }};
}

/// Calculates usage, awaits admission, and returns the calculated value on success.
///
/// Accepts `(catalog, schema, request, rows, source)` or a precomputed `MeterRecord`.
/// Arguments are evaluated once, and the request is borrowed only for calculation.
/// Missing calculators supply zero value while still submitting rows for admission.
#[cfg(not(feature = "noop"))]
#[macro_export]
macro_rules! write_meter {
    ($catalog:expr, $schema:expr, $req_item:expr, $rows:expr, $source:expr) => {{
        let r = meter_core::global::global_registry();
        let item = &$req_item;
        let value = r.get_calculator().map_or(0, |calc| calc.calc(item));
        $crate::write_meter!(meter_core::data::MeterRecord::new(
            $catalog.into(),
            $schema.into(),
            value,
            $rows,
            $source,
        ))
    }};
    ($record:expr) => {{
        let record: meter_core::data::MeterRecord = $record;
        async move {
            let value = record.value;
            meter_core::global::global_registry()
                .record_write(record)
                .await?;
            Ok::<u64, meter_core::collect::WriteRejected>(value)
        }
    }};
}
