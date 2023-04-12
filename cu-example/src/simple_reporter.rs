use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use cu_core::cu_query::CuQuery;
use cu_core::cu_query::RcuCount;
use cu_core::cu_query::WcuCount;
use cu_core::data::ReadRecord;
use cu_core::data::WriteRecord;
use cu_core::reporter::Reporter;
use tracing::info;

pub struct SimpleReporter<C, W, R> {
    cu_query: Arc<C>,
    p1: PhantomData<W>,
    p2: PhantomData<R>,
}

impl<C, W, R> SimpleReporter<C, W, R>
where
    C: CuQuery<W, R>,
    W: Fn(&WriteRecord) -> WcuCount,
    R: Fn(&ReadRecord) -> RcuCount,
{
    pub fn new(cu_query: Arc<C>) -> Self {
        Self {
            cu_query,
            p1: PhantomData,
            p2: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<C, W, R> Reporter for SimpleReporter<C, W, R>
where
    C: CuQuery<W, R>,
    W: Fn(&WriteRecord) -> WcuCount + Send + Sync,
    R: Fn(&ReadRecord) -> RcuCount + Send + Sync,
{
    async fn start(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            info!("===============================================================");

            let wcus = self.cu_query.schema_wcus().unwrap();
            let rcus = self.cu_query.schema_rcus().unwrap();
            self.cu_query.clear();

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
