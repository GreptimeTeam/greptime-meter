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

use std::sync::Arc;

use parking_lot::RwLock;

use crate::collect::Collect;
use crate::data::ReadRecord;
use crate::data::WriteRecord;

#[derive(Default, Clone)]
pub struct Registry {
    inner: Arc<Inner>,
}

#[derive(Default)]
struct Inner {
    collector: RwLock<Option<Arc<dyn Collect>>>,
}

impl Registry {
    /// Set [Collect] for [Registry].
    pub fn set_collector(&self, collector: Arc<dyn Collect>) {
        let mut guard = self.inner.collector.write();
        *guard = Some(collector);
    }
}

impl Registry {
    /// A base API for recording WCU consumption.
    pub fn record_write(&self, record: WriteRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(collector) => collector,
            None => return,
        };

        collector.on_write(record);
    }

    /// A base API for recording RCU consumption.
    pub fn record_read(&self, record: ReadRecord) {
        let collector = self.inner.collector.read();

        let collector = match collector.as_ref() {
            Some(c) => c,
            None => return,
        };

        collector.on_read(record);
    }
}
