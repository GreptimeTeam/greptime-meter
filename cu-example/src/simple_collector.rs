use std::collections::HashMap;

use cu_core::collect::Collect;
use cu_core::cu_query::CuQuery;
use cu_core::cu_query::RcuCount;
use cu_core::cu_query::RegionId;
use cu_core::cu_query::SchemaId;
use cu_core::cu_query::TableId;
use cu_core::cu_query::WcuCount;
use cu_core::data::ReadRecord;
use cu_core::data::WriteRecord;
use cu_core::error::Result;
use dashmap::DashMap;

pub struct SimpleCollector<W, R> {
    read_data: DashMap<SchemaId, Vec<ReadRecord>>,
    write_data: DashMap<SchemaId, Vec<WriteRecord>>,
    wcu_calc: W,
    rcu_calc: R,
}

impl<W, R> SimpleCollector<W, R> {
    pub fn new(wcu_calc: W, rcu_calc: R) -> Self {
        Self {
            read_data: DashMap::default(),
            write_data: DashMap::default(),
            wcu_calc,
            rcu_calc,
        }
    }
}

impl<W, R> CuQuery<W, R> for SimpleCollector<W, R>
where
    R: Fn(&ReadRecord) -> u32 + Send + Sync,
    W: Fn(&WriteRecord) -> u32 + Send + Sync,
{
    fn set_wcu_calc(&mut self, calc: W) {
        self.wcu_calc = calc;
    }

    fn set_rcu_calc(&mut self, calc: R) {
        self.rcu_calc = calc;
    }

    fn clear(&self) {
        self.read_data.clear();
        self.write_data.clear();
    }

    fn schema_wcus(&self) -> Result<HashMap<SchemaId, WcuCount>> {
        Ok(self
            .write_data
            .iter()
            .map(|write_infos| {
                let wcus: u32 = write_infos
                    .value()
                    .iter()
                    .map(|wcu_info| (self.wcu_calc)(wcu_info))
                    .sum();
                (write_infos.key().clone(), wcus)
            })
            .collect())
    }

    fn schema_rcus(&self) -> Result<HashMap<SchemaId, RcuCount>> {
        Ok(self
            .read_data
            .iter()
            .map(|read_infos| {
                let rcus: u32 = read_infos
                    .value()
                    .iter()
                    .map(|read_info| (self.rcu_calc)(read_info))
                    .sum();
                (read_infos.key().clone(), rcus)
            })
            .collect())
    }

    fn region_wcus(&self) -> Result<HashMap<RegionId, WcuCount>> {
        unimplemented!()
    }

    fn region_rcus(&self) -> Result<HashMap<RegionId, WcuCount>> {
        unimplemented!()
    }

    fn table_wcus(&self) -> Result<HashMap<TableId, WcuCount>> {
        unimplemented!()
    }

    fn table_rcus(&self) -> Result<HashMap<TableId, WcuCount>> {
        unimplemented!()
    }
}

impl<W, R> Collect for SimpleCollector<W, R>
where
    R: Send + Sync,
    W: Send + Sync,
{
    fn on_read(&self, record: ReadRecord) {
        let schema_id = SchemaId {
            catalog: record.catalog.clone(),
            schema: record.schema.clone(),
        };

        let mut entry = self.read_data.entry(schema_id).or_insert_with(Vec::new);

        entry.push(record)
    }

    fn on_write(&self, record: WriteRecord) {
        let schema_id = SchemaId {
            catalog: record.catalog.clone(),
            schema: record.schema.clone(),
        };

        let mut entry = self.write_data.entry(schema_id).or_insert_with(Vec::new);

        entry.push(record)
    }
}
