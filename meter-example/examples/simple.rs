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

use meter_core::data::TrafficSource;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use meter_core::data::MeterRecord;
use meter_core::data::ReadItem;
use meter_core::global::global_registry;

use meter_core::ItemCalculator;
use meter_example::collector::SimpleCollector;
use meter_example::reporter::SimpleReporter;
use meter_example::CalcImpl;
use meter_example::MockInsertRequest;
use meter_macros::read_meter;
use meter_macros::write_meter;

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder().finish())
        .unwrap();

    run();
}

#[tokio::main]
async fn run() {
    setup_global_registry().await;

    do_some_record().await;
}

async fn setup_global_registry() {
    let collector = Arc::new(SimpleCollector::new(w_calc, r_calc));
    let reporter = Arc::new(SimpleReporter::new(collector.clone()));

    let r = global_registry();
    r.set_collector(collector);

    let calc_impl = Arc::new(CalcImpl);
    let string_insert_calc = calc_impl.clone() as Arc<dyn ItemCalculator<String>>;
    r.register_calculator(string_insert_calc);

    let mock_insert_calc = calc_impl.clone() as Arc<dyn ItemCalculator<MockInsertRequest>>;
    r.register_calculator(mock_insert_calc);

    let read_item_calc = calc_impl as Arc<dyn ItemCalculator<ReadItem>>;
    r.register_calculator(read_item_calc);

    tokio::spawn(async move {
        reporter.start().await;
    });
}

async fn do_some_record() {
    for _i in 0..20 {
        let insert_req = "String insert req".to_string();
        let w = write_meter!("greptime", "db1", insert_req, TrafficSource::Other);
        info!("w: {}", w);

        let r = read_meter!(
            "greptime",
            "db1",
            ReadItem {
                cpu_time: 100000,
                table_scan: 100000,
            },
            TrafficSource::Other
        );
        info!("r: {}", r);

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn w_calc(w_info: &MeterRecord) -> u64 {
    let MeterRecord { value, .. } = w_info;
    *value
}

fn r_calc(r_info: &MeterRecord) -> u64 {
    let MeterRecord { value, .. } = r_info;
    *value
}
