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

/// Trait representing how to calculate the byte count of a custom type.
/// There are two ways to calculate the byte count of a custom type:
/// 1. Implement `From<&CustomType> for u32` to get byte count
/// 2. Implement `WriteCalculator<CustomType>` to hold a calculator to do the counting
/// Use either way to your condition.
///
/// see `meter-macros` crate for more details.
pub trait WriteCalculator<T>: Send + Sync {
    fn calc_byte(&self, value: &T) -> u32;
}
