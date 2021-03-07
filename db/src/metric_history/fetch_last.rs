use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Fetch<FetchMetricsLastReq> for MetricHistoryCache {
    type Response = FetchMetricsLastRes;

    async fn fetch(&self, input: FetchMetricsLastReq) -> Self::Response {
        let entries = self.0
            .read()
            .await;
        if let Some(x) = entries.get(&input.0) {
            // If the item exist, there is at least one element in the vec
            FetchMetricsLastRes::Ok(x.clone().pop().unwrap())
        } else {
            FetchMetricsLastRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchLast)]
#[derive(Debug, Parse)]
pub struct FetchMetricsLastReq(pub Uuid);

#[derive(Debug, Parse)]
pub enum FetchMetricsLastRes {
    Ok(MetricHistoryEntry),
    Err(EmptyMsg),
}

#[cfg(test)]
mod metric_history_fetch_tests {
    use super::*;

    use std::collections::HashMap;
    use tokio::sync::RwLock;

    /// Has a set of values, only the lastest will be returned
    #[tokio::test]
    async fn fetch_01() {
        let uuid_0 = Uuid::new_v4();
        let mut entries = HashMap::new();
        entries.insert(uuid_0, vec![
            MetricHistoryEntry { timestamp: 0u64, value: 0u128 },
            MetricHistoryEntry { timestamp: 1u64, value: 2u128 },
            MetricHistoryEntry { timestamp: 3u64, value: 4u128 },
            MetricHistoryEntry { timestamp: 5u64, value: 6u128 },
        ]);
        let cache = MetricHistoryCache(RwLock::new(entries));

        let input = FetchMetricsLastReq(uuid_0);

        match cache.fetch(input).await {
            FetchMetricsLastRes::Ok(x) => {
                assert_eq!(x.timestamp, 5u64);
                assert_eq!(x.value, 6u128);
            },
            _ => assert!(false)
        }
    }

    // This is not the id you are searching for
    #[tokio::test]
    async fn fetch_02() {
        let uuid_0 = Uuid::new_v4();
        let cache = MetricHistoryCache(RwLock::new(HashMap::new()));

        let input = FetchMetricsLastReq(uuid_0);

        match cache.fetch(input).await {
            FetchMetricsLastRes::Err(_) => assert!(true),
            _ => assert!(false)
        };
    }
}
