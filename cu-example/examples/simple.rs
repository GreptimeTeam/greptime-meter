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

use cu_core::data::ReadRecord;
use cu_core::data::WriteRecord;
use cu_core::global::global_registry;
use cu_core::registry::Registry;
use cu_core::write_calc::WriteCalculator;
use cu_example::collector::SimpleCollector;
use cu_example::reporter::SimpleReporter;
use cu_macros::write_meter;

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder().finish())
        .unwrap();

    run();
}

#[tokio::main]
async fn run() {
    let r = global_registry();

    setup_global_registry().await;

    do_some_record(r).await;
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

async fn do_some_record(r: Registry) {
    for _i in 0..20 {
        let insert_req = "String insert req".to_string();
        write_meter!("greptime", "db1", insert_req);
        write_meter!("greptime", "db1", 1);

        r.record_read(ReadRecord {
            catalog: "greptime".to_string(),
            schema: "db2".to_string(),
            table: None,
            region_num: None,
            cpu_time: 3,
            table_scan: 0,
            network_egress: 0,
        });

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

struct MockInsertRequest;

pub struct CalcImpl;

impl WriteCalculator<MockInsertRequest> for CalcImpl {
    fn calc_byte(&self, _value: &MockInsertRequest) -> u32 {
        1024 * 10
    }
}

impl WriteCalculator<String> for CalcImpl {
    fn calc_byte(&self, _value: &String) -> u32 {
        1024 * 100
    }
}
