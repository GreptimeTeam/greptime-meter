use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use cu_core::data::ReadRecord;
use cu_core::data::WriteRecord;
use tracing::info;

use crate::collector::SimpleCollector;

/// A simple reporter that outputs wrcu information to stdout.
pub struct SimpleReporter<W, R> {
    collector: Arc<SimpleCollector<W, R>>,
    p1: PhantomData<W>,
    p2: PhantomData<R>,
}

impl<W, R> SimpleReporter<W, R> {
    pub fn new(collector: Arc<SimpleCollector<W, R>>) -> Self {
        Self {
            collector,
            p1: PhantomData,
            p2: PhantomData,
        }
    }
}

impl<W, R> SimpleReporter<W, R>
where
    W: Fn(&WriteRecord) -> u32 + Send + Sync,
    R: Fn(&ReadRecord) -> u32 + Send + Sync,
{
    pub async fn start(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            info!("===============================================================");

            let wcus = self.collector.schema_wcus();
            let rcus = self.collector.schema_rcus();
            self.collector.clear();

            info!("The number of WCUs consumed in the last 5 seconds:");
            for (id, wcu_number) in wcus {
                info!(
                    "catalog {}, schema {}, wcus: {}",
                    id.catalog, id.schema, wcu_number
                );
            }

            info!("The number of RCUs consumed in the last 5 seconds:");
            for (id, rcu_number) in rcus {
                info!(
                    "catalog {}, schema {}, rcus: {}",
                    id.catalog, id.schema, rcu_number
                );
            }
        }
    }
}
