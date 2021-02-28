use super::{MetricInfoCache, MetricInfoEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyMsg, Lookup, Parse, Storage, request};
use uuid::Uuid;

/// Resolves the given name to a uuid
///
/// If the name is not in the cache it will be added and the uuid will be returned
#[async_trait]
impl Lookup<LookupMetricIdReq> for MetricInfoCache {
    type Error    = EmptyMsg;
    type Response = LookupMetricIdRes;

    async fn lookup(&self, input: LookupMetricIdReq) -> Result<Self::Response, Self::Error> {
        // Try to get the key from the map
        let id = self.0
            .read()
            .await
            .iter()
            .find(|(_, e)| e.key == input.0)
            .map(|(_, e)| e.id);

        // If the id is in the map, return the id
        // If the id is not in the map, generate a new uuid and insert it
        // and return it
        let id = if let Some(id) = id {
            id
        } else {
            let entry = MetricInfoEntry::new(input.0.clone());
            let id = entry.id;
            self.0
                .write()
                .await
                .insert(entry.id, entry);
            id
        };

        self.save_to_file().await.unwrap();
        Ok(LookupMetricIdRes(LookupMetricEntry {
            key: input.0,
            id,
        }))
    }
}

#[request(Actions::LookupMetricId)]
#[derive(Debug, Parse)]
pub struct LookupMetricIdReq(pub String);

#[derive(Debug, Parse)]
pub struct LookupMetricIdRes(pub LookupMetricEntry);

#[derive(Debug, Parse)]
pub struct LookupMetricEntry {
    pub key: String,
    pub id: Uuid,
}
