use std::collections::HashMap;

use cu_core::collect::Collect;
use cu_core::data::ReadRecord;
use cu_core::data::WriteRecord;
use dashmap::DashMap;

pub struct SimpleCollector<W, R> {
    read_data: DashMap<SchemaId, Vec<ReadRecord>>,
    write_data: DashMap<SchemaId, Vec<WriteRecord>>,
    wcu_calc: W,
    rcu_calc: R,
}

/// The SchemaId identifies a database.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct SchemaId {
    pub catalog: String,
    pub schema: String,
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

impl<W, R> SimpleCollector<W, R>
where
    R: Fn(&ReadRecord) -> u32 + Send + Sync,
    W: Fn(&WriteRecord) -> u32 + Send + Sync,
{
    pub fn clear(&self) {
        self.read_data.clear();
        self.write_data.clear();
    }

    pub fn schema_wcus(&self) -> HashMap<SchemaId, u32> {
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

    pub fn schema_rcus(&self) -> HashMap<SchemaId, u32> {
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