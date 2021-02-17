use super::{MetricInfoCache, MetricInfoEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyResponse, Lookup, Parse, Storage, request};
use uuid::Uuid;

/// Resolves the given name to a uuid
///
/// If the name is not in the cache it will be added and the uuid will be returned
#[async_trait]
impl Lookup<LookupMetricIdsReq> for MetricInfoCache {
    type Error    = EmptyResponse;
    type Response = LookupMetricIdsRes;

    async fn lookup(&self, input: LookupMetricIdsReq) -> Result<Self::Response, Self::Error> {
        let mut ids = Vec::with_capacity(input.0.len());

        for key in input.0 {
            // Try to get the key from the map
            let id = self.0
                .read()
                .await
                .iter()
                .find(|(_, e)| e.key == key)
                .map(|(_, e)| e.id);

            // If the id is in the map, return the id
            // If the id is not in the map, generate a new uuid and insert it
            // and return it
            let id = if let Some(id) = id {
                id
            } else {
                let entry = MetricInfoEntry::new(key.clone());
                let id = entry.id;
                self.0
                    .write()
                    .await
                    .insert(entry.id, entry);
                id
            };
            ids.push(LookupMetricEntry {
                key,
                id,
            });
        }

        self.save_to_file().await.unwrap();
        Ok(LookupMetricIdsRes(ids))
    }
}

#[request(Actions::LookupMetricId)]
#[derive(Debug, Parse)]
pub struct LookupMetricIdsReq(pub Vec<String>);

// TODO: support derive with tuples
// TODO: support derive Vec<&'static str>
#[derive(Debug, Parse)]
pub struct LookupMetricIdsRes(pub Vec<LookupMetricEntry>);

#[derive(Debug, Parse)]
pub struct LookupMetricEntry {
    pub key: String,
    pub id: Uuid,
}
