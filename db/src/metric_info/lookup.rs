use super::{MetricInfoCache, MetricInfoEntry};

use crate::{Actions, Caches, EmptyResponse};

use async_trait::*;
use cachem::{Lookup, Parse, request};
use uuid::Uuid;

/// Resolves the given name to a uuid
///
/// If the name is not in the cache it will be added and the uuid will be returned
#[async_trait]
impl Lookup<LookupMetricUuid> for MetricInfoCache {
    type Error = EmptyResponse;
    type Response = Uuid;
    async fn lookup(&self, input: LookupMetricUuid) -> Result<Self::Response, Self::Error> {
        let key = input.0;
        let id = self.0
            .read()
            .await
            .iter()
            .find(|(_, e)| e.key == key)
            .map(|(_, e)| e.id);

        if let Some(id) = id {
            Ok(id)
        } else {
            let entry = MetricInfoEntry::new(key);
            let id = entry.id;
            self.0
                .write()
                .await
                .insert(entry.id, entry);
            Ok(id)
        }
    }
}

#[request(Actions::Resolve, Caches::MetricInfo)]
#[derive(Debug, Parse)]
pub struct LookupMetricUuid(pub String);

#[derive(Debug, Parse)]
pub struct LookupMetricUuidResponse(pub Vec<Uuid>);
