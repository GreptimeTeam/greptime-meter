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
use std::time::Duration;

use meter_core::data::ReadRecord;
use meter_core::data::WriteRecord;
use meter_core::global::global_registry;
use meter_core::write_calc::WriteCalculator;
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
    let collector = Arc::new(SimpleCollector::new(wcu_calc, rcu_calc));
    let reporter = Arc::new(SimpleReporter::new(collector.clone()));

    let r = global_registry();
    r.set_collector(collector);

    let calc_impl = Arc::new(CalcImpl);
    let string_insert_calc = calc_impl.clone() as Arc<dyn WriteCalculator<String>>;
    r.register_calculator(string_insert_calc);

    let mock_insert_calc = calc_impl as Arc<dyn WriteCalculator<MockInsertRequest>>;
    r.register_calculator(mock_insert_calc);

    tokio::spawn(async move {
        reporter.start().await;
    });
}

async fn do_some_record() {
    for _i in 0..20 {
        let insert_req = "String insert req".to_string();
        write_meter!("greptime", "db1", insert_req);

        read_meter!("greptime", "db1", cpu_time: 100000);
        read_meter!("greptime", "db1", table_scan: 100000);
        read_meter!("greptime", "db1", network_egress: 100000);

        read_meter!("greptime", "db2", 100000, 100000, 100000);

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn wcu_calc(w_info: &WriteRecord) -> u32 {
    let WriteRecord { byte_count, .. } = w_info;

    byte_count / 1024
}

fn rcu_calc(r_info: &ReadRecord) -> u32 {
    let ReadRecord {
        cpu_time,
        table_scan,
        network_egress,
        ..
    } = r_info;

    *cpu_time / 3 + table_scan / 4096 + network_egress / 4096
}
