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

use std::collections::HashMap;

use dashmap::DashMap;
use meter_core::collect::Collect;
use meter_core::data::MeterRecord;

pub struct SimpleCollector<W, R> {
    read_data: DashMap<SchemaId, Vec<MeterRecord>>,
    write_data: DashMap<SchemaId, Vec<MeterRecord>>,
    w_calc: W,
    r_calc: R,
}

/// The SchemaId identifies a database.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct SchemaId {
    pub catalog: String,
    pub schema: String,
}

impl<W, R> SimpleCollector<W, R> {
    pub fn new(w_calc: W, r_calc: R) -> Self {
        Self {
            read_data: DashMap::default(),
            write_data: DashMap::default(),
            w_calc,
            r_calc,
        }
    }
}

impl<W, R> SimpleCollector<W, R>
where
    R: Fn(&MeterRecord) -> u64 + Send + Sync,
    W: Fn(&MeterRecord) -> u64 + Send + Sync,
{
    pub fn clear(&self) {
        self.read_data.clear();
        self.write_data.clear();
    }

    pub fn schema_ws(&self) -> HashMap<SchemaId, u64> {
        self.write_data
            .iter()
            .map(|write_infos| {
                let ws: u64 = write_infos
                    .value()
                    .iter()
                    .map(|w_info| (self.w_calc)(w_info))
                    .sum();
                (write_infos.key().clone(), ws)
            })
            .collect()
    }

    pub fn schema_rs(&self) -> HashMap<SchemaId, u64> {
        self.read_data
            .iter()
            .map(|read_infos| {
                let rs: u64 = read_infos
                    .value()
                    .iter()
                    .map(|read_info| (self.r_calc)(read_info))
                    .sum();
                (read_infos.key().clone(), rs)
            })
            .collect()
    }
}

impl<W, R> Collect for SimpleCollector<W, R>
where
    R: Send + Sync,
    W: Send + Sync,
{
    fn on_read(&self, record: MeterRecord) {
        let schema_id = SchemaId {
            catalog: record.catalog.clone(),
            schema: record.schema.clone(),
        };

        let mut entry = self.read_data.entry(schema_id).or_default();

        entry.push(record)
    }

    fn on_write(&self, record: MeterRecord) {
        let schema_id = SchemaId {
            catalog: record.catalog.clone(),
            schema: record.schema.clone(),
        };

        let mut entry = self.write_data.entry(schema_id).or_default();

        entry.push(record)
    }
}
