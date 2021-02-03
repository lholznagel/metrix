use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::{Actions, Caches, EmptyResponse};

use async_trait::*;
use cachem::{Insert, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Insert<InsertMetricsReq> for MetricHistoryCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertMetricsReq) -> Result<Self::Response, Self::Error> {
        let mut data_copy = self.0.read().await.clone();
        for x in input.0 {
            let entry = MetricHistoryEntry {
                timestamp: x.ts,
                value: x.value,
            };

            data_copy
                .entry(x.id)
                .and_modify(|x| x.push(entry))
                .or_insert(vec![entry]);
        }
        *self.0.write().await = data_copy;

        Ok(EmptyResponse::default())
    }
}
#[request(Actions::Insert, Caches::MetricHistory)]
#[derive(Debug, Parse)]
pub struct InsertMetricsReq(pub Vec<InsertMetricsEntry>);

#[derive(Clone, Copy, Debug, Parse)]
pub struct InsertMetricsEntry {
    /// Id of the entry
    pub id: Uuid,
    /// Timestamp when the given value happened
    pub ts: u128,
    /// Value that maps to the id and timestamp
    pub value: u128,
}

#[cfg(test)]
mod metric_history_insert_tests {
    use super::*;

    use std::collections::HashMap;
    use tokio::sync::RwLock;

    /// Has one value stored and sends another value
    #[tokio::test]
    async fn insert_01() {
        let uuid_0 = Uuid::new_v4();
        let mut entries = HashMap::new();
        entries.insert(uuid_0, vec![
            MetricHistoryEntry { timestamp: 0u128, value: 0u128 },
        ]);
        let cache = MetricHistoryCache(RwLock::new(entries));
        assert!(cache.0.read().await.get(&uuid_0).unwrap().len() == 1);

        let input_data = vec![
            InsertMetricsEntry { id: uuid_0, ts: 1, value: 1 }
        ];
        let input = InsertMetricsReq(input_data);
        let res = cache.insert(input).await;
        assert!(res.is_ok());
        assert!(cache.0.read().await.get(&uuid_0).unwrap().len() == 2);
    }
}

