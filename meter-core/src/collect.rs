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

use std::fmt;
use std::future::Future;
use std::pin::Pin;

use crate::data::MeterRecord;

/// A write rejected by the collector's admission policy.
#[derive(Debug)]
pub struct WriteRejected {
    pub reason: String,
}

impl WriteRejected {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

impl fmt::Display for WriteRejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for WriteRejected {}

/// Trait representing the methods required to collect read/write record.
/// Save read and write separately for later refactoring.
pub trait Collect: Send + Sync {
    /// Admits a write and records its accepted usage before write dispatch.
    ///
    /// Rejection must happen before recording accepted usage. Success does not
    /// imply persistence; the registry does not acquire quota or roll it back
    /// if downstream writing fails.
    fn on_write(
        &self,
        record: MeterRecord,
    ) -> Pin<Box<dyn Future<Output = Result<(), WriteRejected>> + Send + '_>>;

    /// Notifies the method that an event about data query occurs.
    fn on_read(&self, record: MeterRecord);
}
