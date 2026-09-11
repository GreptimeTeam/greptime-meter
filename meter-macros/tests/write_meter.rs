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

use std::future::Future;
use std::pin::pin;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;

use meter_core::collect::Collect;
use meter_core::collect::WriteRejected;
use meter_core::data::MeterRecord;
use meter_core::global::global_registry;
use meter_macros::read_meter;
use meter_macros::write_meter;

#[derive(Default)]
struct Collector(Mutex<Vec<MeterRecord>>);

impl Collect for Collector {
    fn on_write(
        &self,
        record: MeterRecord,
    ) -> Pin<Box<dyn Future<Output = Result<(), WriteRejected>> + Send + '_>> {
        Box::pin(async move {
            if record.schema == "rejected" {
                return Err(WriteRejected::new("row quota exceeded"));
            }
            self.0.lock().unwrap().push(record);
            Ok(())
        })
    }

    fn on_read(&self, record: MeterRecord) {
        self.0.lock().unwrap().push(record);
    }
}

fn complete<T>(future: impl Future<Output = T> + Send) -> T {
    match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("unexpected suspension"),
    }
}

// One test per feature configuration owns the process-global registry.
#[cfg(not(feature = "noop"))]
#[test]
fn enabled_admission_and_read_contracts() {
    use std::cell::Cell;

    use meter_core::data::ReadItem;
    use meter_core::ItemCalculator;

    struct Calculator;
    impl ItemCalculator<String> for Calculator {
        fn calc(&self, request: &String) -> u64 {
            request.len() as u64
        }
    }
    impl ItemCalculator<ReadItem> for Calculator {
        fn calc(&self, item: &ReadItem) -> u64 {
            item.cpu_time
        }
    }

    let registry = global_registry();
    let request = "request".to_string();
    assert_eq!(
        complete(write_meter!("catalog", "schema", request, 2, 3)).unwrap(),
        0
    );
    registry.register_calculator(Arc::new(Calculator) as Arc<dyn ItemCalculator<String>>);
    assert_eq!(
        complete(write_meter!("catalog", "schema", request, 2, 3)).unwrap(),
        7
    );

    let collector = Arc::new(Collector::default());
    registry.set_collector(collector.clone());
    assert_eq!(
        complete(write_meter!("catalog", "schema", request, 2, 3)).unwrap(),
        7
    );
    assert_eq!(request, "request"); // The insertion request is still available for dispatch.

    let calls = Cell::new(0);
    let once = |value| {
        calls.set(calls.get() + 1);
        value
    };
    assert_eq!(
        complete(write_meter!(
            once("c"),
            once("s"),
            once("abc").to_string(),
            {
                calls.set(calls.get() + 1);
                4
            },
            {
                calls.set(calls.get() + 1);
                5
            }
        ))
        .unwrap(),
        3
    );
    assert_eq!(calls.get(), 5);

    struct UnknownRequest;
    assert_eq!(
        complete(write_meter!(
            "catalog",
            "schema",
            UnknownRequest,
            u64::MAX,
            6
        ))
        .unwrap(),
        0
    );
    assert_eq!(
        complete(write_meter!("catalog", "schema", String::new(), 3, 7)).unwrap(),
        0
    );
    assert_eq!(
        complete(write_meter!(MeterRecord::new(
            "bulk".into(),
            "schema".into(),
            19,
            8,
            9
        )))
        .unwrap(),
        19
    );

    for result in [
        complete(write_meter!("catalog", "rejected", request, 2, 3)),
        complete(write_meter!("catalog", "rejected", UnknownRequest, 2, 3)),
        complete(write_meter!("catalog", "rejected", String::new(), 2, 3)),
        complete(write_meter!(MeterRecord::new(
            "bulk".into(),
            "rejected".into(),
            0,
            8,
            9
        ))),
    ] {
        assert_eq!(result.unwrap_err().reason, "row quota exceeded");
    }

    registry.register_calculator(Arc::new(Calculator) as Arc<dyn ItemCalculator<ReadItem>>);
    let cost: u64 = read_meter!("read_catalog", "read_schema", ReadItem::new(13, 20), 10);
    assert_eq!(cost, 13);
    let records = collector.0.lock().unwrap();
    let actual: Vec<_> = records
        .iter()
        .map(|r| {
            (
                r.catalog.as_str(),
                r.schema.as_str(),
                r.value,
                r.rows,
                r.source,
            )
        })
        .collect();
    assert_eq!(
        actual,
        vec![
            ("catalog", "schema", 7, 2, 3),
            ("c", "s", 3, 4, 5),
            ("catalog", "schema", 0, u64::MAX, 6),
            ("catalog", "schema", 0, 3, 7),
            ("bulk", "schema", 19, 8, 9),
            ("read_catalog", "read_schema", 13, 0, 10),
        ]
    );
}

#[cfg(feature = "noop")]
#[test]
fn noop_skips_evaluation_and_admission() {
    let collector = Arc::new(Collector::default());
    global_registry().set_collector(collector.clone());
    let request = "request".to_string();
    let mut evaluations = 0;
    let mut rows = || {
        evaluations += 1;
        12
    };
    assert_eq!(
        complete(write_meter!("catalog", "rejected", request, rows(), 3)).unwrap(),
        0
    );
    assert_eq!(request, "request");
    assert_eq!(
        complete(write_meter!({
            evaluations += 1;
            MeterRecord::new("bulk".into(), "rejected".into(), 19, 8, 9)
        }))
        .unwrap(),
        0
    );
    assert_eq!(evaluations, 0);
    let cost: u64 = read_meter!(
        "catalog",
        "schema",
        meter_core::data::ReadItem::new(13, 20),
        10
    );
    assert_eq!(cost, 0);
    assert!(collector.0.lock().unwrap().is_empty());
}
