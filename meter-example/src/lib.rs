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

use meter_core::data::ReadItem;
use meter_core::ItemCalculator;

pub mod collector;
pub mod reporter;

pub struct MockInsertRequest;

pub struct CalcImpl;

impl ItemCalculator<MockInsertRequest> for CalcImpl {
    fn calc(&self, _value: &MockInsertRequest) -> u64 {
        1024
    }
}

impl ItemCalculator<String> for CalcImpl {
    fn calc(&self, _value: &String) -> u64 {
        1024 * 10
    }
}

impl ItemCalculator<ReadItem> for CalcImpl {
    fn calc(&self, _value: &ReadItem) -> u64 {
        1024 * 100
    }
}
