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

use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use meter_core::data::MeterRecord;
use tracing::info;

use crate::collector::SimpleCollector;

/// A simple reporter that outputs w/r information to stdout.
pub struct SimpleReporter<W, R> {
    collector: Arc<SimpleCollector<W, R>>,
    p1: PhantomData<W>,
    p2: PhantomData<R>,
}

impl<W, R> SimpleReporter<W, R> {
    pub fn new(collector: Arc<SimpleCollector<W, R>>) -> Self {
        Self {
            collector,
            p1: PhantomData,
            p2: PhantomData,
        }
    }
}

impl<W, R> SimpleReporter<W, R>
where
    W: Fn(&MeterRecord) -> u64 + Send + Sync,
    R: Fn(&MeterRecord) -> u64 + Send + Sync,
{
    pub async fn start(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            info!("===============================================================");

            let ws = self.collector.schema_ws();
            let rs = self.collector.schema_rs();
            self.collector.clear();

            info!("The number of Ws consumed in the last 5 seconds:");
            for (id, w_number) in ws {
                info!(
                    "catalog {}, schema {}, ws: {}",
                    id.catalog, id.schema, w_number
                );
            }

            info!("The number of Rs consumed in the last 5 seconds:");
            for (id, r_number) in rs {
                info!(
                    "catalog {}, schema {}, rs: {}",
                    id.catalog, id.schema, r_number
                );
            }
        }
    }
}
