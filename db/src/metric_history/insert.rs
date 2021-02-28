use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyMsg, Insert, Parse, request};
use chrono::Utc;
use uuid::Uuid;

#[async_trait]
impl Insert<InsertMetricsReq> for MetricHistoryCache {
    type Error    = EmptyMsg;
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertMetricsReq) -> Result<Self::Response, Self::Error> {
        let timestamp = Utc::now().timestamp_nanos() as u64;
        let metric = input.0;
        let mut data_copy = self.0.read().await.clone();

        // Create the new entry
        let entry = MetricHistoryEntry {
            timestamp,
            value: metric.value,
        };

        // Check if the last value is the same as the current one
        if let Some(x) = data_copy.get(&metric.id) {
            if x.last().unwrap().value != metric.value {
                // If the id already has entries append the new entry
                // If the id is new create it and add the first entry
                data_copy
                    .entry(metric.id)
                    .and_modify(|x| x.push(entry))
                    .or_insert(vec![entry]);
            }
        } else {
            // If the id already has entries append the new entry
            // If the id is new create it and add the first entry
            data_copy
                .entry(metric.id)
                .and_modify(|x| x.push(entry))
                .or_insert(vec![entry]);
        }

        *self.0.write().await = data_copy;
        Ok(EmptyMsg::default())
    }
}

#[request(Actions::InsertMetric)]
#[derive(Debug, Parse)]
pub struct InsertMetricsReq(pub InsertMetricsEntry);

#[derive(Clone, Copy, Debug, Parse)]
pub struct InsertMetricsEntry {
    /// Id of the entry
    pub id: Uuid,
    /// Value for that id
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
            MetricHistoryEntry { timestamp: 0u64, value: 0u128 },
        ]);
        let cache = MetricHistoryCache(RwLock::new(entries));
        assert!(cache.0.read().await.get(&uuid_0).unwrap().len() == 1);

        let input_data = InsertMetricsEntry { id: uuid_0, value: 1 };
        let input = InsertMetricsReq(input_data);
        let res = cache.insert(input).await;
        assert!(res.is_ok());
        assert!(cache.0.read().await.get(&uuid_0).unwrap().len() == 2);
    }
}

