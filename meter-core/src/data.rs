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

#[derive(Debug)]
pub struct ReadItem {
    /// The CPU consumed by query SQL processes.
    ///
    /// Unit is nanosecond.
    pub cpu_time: u64,

    /// The data size of table scan plan.
    ///
    /// Unit is byte.
    pub table_scan: u64,
}

impl ReadItem {
    pub fn new(cpu_time: u64, table_scan: u64) -> Self {
        Self {
            cpu_time,
            table_scan,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum TrafficSource {
    #[default]
    Other = 0u8,

    Prometheus = 1u8,
    Influx = 2u8,
    HTTP = 3u8,
    MySQL = 4u8,
    Postgres = 5u8,
    GRPC = 6u8,
    OTLP = 7u8,
}

impl From<u8> for TrafficSource {
    fn from(value: u8) -> Self {
        match value {
            1 => TrafficSource::Prometheus,
            2 => TrafficSource::Influx,
            3 => TrafficSource::HTTP,
            4 => TrafficSource::MySQL,
            5 => TrafficSource::Postgres,
            6 => TrafficSource::GRPC,
            7 => TrafficSource::OTLP,

            _ => TrafficSource::Other,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MeterRecord {
    pub catalog: String,
    pub schema: String,
    pub value: u64,
    pub source: TrafficSource,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traffic_source_roundtrip() {
        assert_eq!(
            TrafficSource::Prometheus,
            TrafficSource::from(TrafficSource::Prometheus as u8)
        );
        assert_eq!(
            TrafficSource::Influx,
            TrafficSource::from(TrafficSource::Influx as u8)
        );
        assert_eq!(
            TrafficSource::MySQL,
            TrafficSource::from(TrafficSource::MySQL as u8)
        );
        assert_eq!(
            TrafficSource::Postgres,
            TrafficSource::from(TrafficSource::Postgres as u8)
        );
        assert_eq!(
            TrafficSource::GRPC,
            TrafficSource::from(TrafficSource::GRPC as u8)
        );
        assert_eq!(
            TrafficSource::OTLP,
            TrafficSource::from(TrafficSource::OTLP as u8)
        );
        assert_eq!(
            TrafficSource::Other,
            TrafficSource::from(TrafficSource::Other as u8)
        );
        assert_eq!(TrafficSource::Other, TrafficSource::from(100u8));
    }
}
