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

/// The WriteRecord records some data about data insertion.
#[derive(Debug)]
pub struct WriteRecord {
    pub catalog: String,
    pub schema: String,
    pub table: Option<String>,
    pub region_num: Option<u32>,

    /// Volume of data written in byte.
    pub byte_count: u32,

    /// The calculated value of this read record.
    ///
    /// If present, other fields will be ignored.
    pub calculated_value: Option<u64>,
}

/// The ReadRecord records some data about data query.
#[derive(Debug)]
pub struct ReadRecord {
    pub catalog: String,
    pub schema: String,
    pub table: Option<String>,
    pub region_num: Option<u32>,

    /// The CPU consumed by query SQL processes.
    ///
    /// Unit is nanosecond.
    pub cpu_time: u64,

    /// The data size of table scan plan.
    ///
    /// Unit is byte.
    pub table_scan: u64,

    /// The size of the network traffic used by the query.
    ///
    /// Unit is byte.
    pub network_egress: u64,

    /// The calculated value of this read record.
    ///
    /// If present, other fields will be ignored.
    pub calculated_value: Option<u64>,
}
