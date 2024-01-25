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

/// Trait representing how to calculate the byte count of a custom type `T`.
/// Implement `WriteCalculator<T>` and register it to [`crate::registry::Registry`]
/// and then you can use `write_meter!()` with input of type `T` directly.
///
/// see `meter-macros` crate for example.
pub trait WriteCalculator<T>: Send + Sync {
    fn calc_byte(&self, value: &T) -> u32;
}
