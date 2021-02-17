use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Fetch<FetchMetricsLastReq> for MetricHistoryCache {
    type Error    = EmptyResponse;
    type Response = FetchMetricsLastRes;

    async fn fetch(&self, input: FetchMetricsLastReq) -> Result<Self::Response, Self::Error> {
        let entries = self.0
            .read()
            .await;
        if let Some(x) = entries.get(&input.0) {
            // If the item exist, there is at least one element in the vec
            return Ok(FetchMetricsLastRes(x.clone().pop().unwrap()))
        }

        Err(EmptyResponse::default())
    }
}

#[request(Actions::FetchLast)]
#[derive(Debug, Parse)]
pub struct FetchMetricsLastReq(pub Uuid);

#[derive(Debug, Parse)]
pub struct FetchMetricsLastRes(pub MetricHistoryEntry);

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

        let res = cache.fetch(input).await;
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res.0.timestamp, 5u64);
        assert_eq!(res.0.value, 6u128);
    }

    // This is not the id you are searching for
    #[tokio::test]
    async fn fetch_02() {
        let uuid_0 = Uuid::new_v4();
        let cache = MetricHistoryCache(RwLock::new(HashMap::new()));

        let input = FetchMetricsLastReq(uuid_0);

        let res = cache.fetch(input).await;
        assert!(res.is_err());
    }
}
