use std::{sync::Arc, time::Duration};

use cu_core::{
    data::{ReadRecord, Scenes, WriteRecord},
    global::global_registry,
    registry::Registry,
};
use cu_example::{simple_collector::SimpleCollector, simple_reporter::SimpleReporter};

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

    let mut r = global_registry();
    r.set_reporter(reporter);
    r.set_collector(collector);

    tokio::spawn(async move {
        r.start().await;
    });
}

async fn do_some_record(r: Registry) {
    for _i in 0..20 {
        r.record_write(WriteRecord {
            catalog: "greptime".to_string(),
            schema: "db1".to_string(),
            table: None,
            region_num: None,
            byte_count: 10 * 1024,
            scenes: Scenes::RateLimit,
        });

        r.record_read(ReadRecord {
            catalog: "greptime".to_string(),
            schema: "db2".to_string(),
            table: None,
            region_num: None,
            cpu_time: 3,
            table_scan: 0,
            network_egress: 0,
            scenes: Scenes::RateLimit,
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
