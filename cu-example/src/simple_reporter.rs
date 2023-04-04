use std::{marker::PhantomData, sync::Arc, time::Duration};

use cu_core::{
    cu_query::{CuQuery, RcuCount, WcuCount},
    data::{ReadRecord, WriteRecord},
    reporter::Reporter,
};
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

            let wcus = self.cu_query.service_wcus();
            let rcus = self.cu_query.service_rcus();
            self.cu_query.clear();

            info!("The number of WCUs consumed in the last 5 seconds:");
            for (id, wcu_numer) in wcus {
                info!(
                    "catalog {}, schema {}, wcus: {}",
                    id.catalog, id.schema, wcu_numer
                );
            }

            info!("The number of RCUs consumed in the last 5 seconds:");
            for (id, rcu_numer) in rcus {
                info!(
                    "catalog {}, schema {}, rcus: {}",
                    id.catalog, id.schema, rcu_numer
                );
            }
        }
    }
}
