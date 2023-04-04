use std::collections::HashMap;

use cu_core::{
    collect::Collect,
    cu_query::{CuQuery, RcuCount, WcuCount},
    data::{ReadRecord, RegionId, Scenes, ServiceId, TableId, WriteRecord},
};
use dashmap::DashMap;

pub struct SimpleCollector<W, R> {
    read_data: DashMap<ServiceId, Vec<ReadRecord>>,
    write_data: DashMap<ServiceId, Vec<WriteRecord>>,
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

    fn service_wcus(&self) -> HashMap<ServiceId, WcuCount> {
        self.write_data
            .iter()
            .map(|write_infos| {
                let wcus: u32 = write_infos
                    .value()
                    .iter()
                    .map(|wcu_info| (self.wcu_calc)(wcu_info))
                    .sum();
                (write_infos.key().clone(), wcus)
            })
            .collect()
    }

    fn service_rcus(&self) -> HashMap<ServiceId, RcuCount> {
        self.read_data
            .iter()
            .map(|read_infos| {
                let rcus: u32 = read_infos
                    .value()
                    .iter()
                    .map(|read_info| (self.rcu_calc)(read_info))
                    .sum();
                (read_infos.key().clone(), rcus)
            })
            .collect()
    }

    fn region_wcus(&self) -> HashMap<RegionId, WcuCount> {
        unimplemented!()
    }

    fn region_rcus(&self) -> HashMap<RegionId, WcuCount> {
        unimplemented!()
    }

    fn table_wcus(&self) -> HashMap<TableId, WcuCount> {
        unimplemented!()
    }

    fn table_rcus(&self) -> HashMap<TableId, WcuCount> {
        unimplemented!()
    }
}

impl<W, R> Collect for SimpleCollector<W, R>
where
    R: Send + Sync,
    W: Send + Sync,
{
    fn on_read(&self, record: ReadRecord) {
        if let Scenes::DistributedScheduling = record.scenes {
            unimplemented!()
        }

        let service_id = ServiceId {
            catalog: record.catalog.clone(),
            schema: record.schema.clone(),
        };

        let mut entry = self.read_data.entry(service_id).or_insert_with(Vec::new);

        entry.push(record)
    }

    fn on_write(&self, record: WriteRecord) {
        if let Scenes::DistributedScheduling = record.scenes {
            unimplemented!()
        }

        let service_id = ServiceId {
            catalog: record.catalog.clone(),
            schema: record.schema.clone(),
        };

        let mut entry = self.write_data.entry(service_id).or_insert_with(Vec::new);

        entry.push(record)
    }
}
