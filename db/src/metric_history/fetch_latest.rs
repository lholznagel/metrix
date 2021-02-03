use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::{Actions, Caches, EmptyResponse};

use async_trait::*;
use cachem::{Fetch, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Fetch<FetchMetricsLatestReq> for MetricHistoryCache {
    type Error    = EmptyResponse;
    type Response = FetchMetricsLatestRes;

    async fn fetch(&self, input: FetchMetricsLatestReq) -> Result<Self::Response, Self::Error> {
        let filter = input.0;
        let entries = self.0
            .read()
            .await;
        if let Some(x) = entries.get(&filter.id) {
            // If the item exist, there is at least one element in the vec
            return Ok(FetchMetricsLatestRes(x.clone().pop().unwrap()))
        }

        Err(EmptyResponse::default())
    }
}

#[request(Actions::Fetch, Caches::MetricHistory)]
#[derive(Debug, Parse)]
pub struct FetchMetricsLatestReq(pub FetchMetricLatestFilter);

#[derive(Debug, Parse)]
pub struct FetchMetricsLatestRes(pub MetricHistoryEntry);

#[derive(Clone, Copy, Debug, Parse)]
pub struct FetchMetricLatestFilter {
    /// Id of the requested metric
    pub id: Uuid,
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
            MetricHistoryEntry { timestamp: 0u128, value: 0u128 },
            MetricHistoryEntry { timestamp: 1u128, value: 2u128 },
            MetricHistoryEntry { timestamp: 3u128, value: 4u128 },
            MetricHistoryEntry { timestamp: 5u128, value: 6u128 },
        ]);
        let cache = MetricHistoryCache(RwLock::new(entries));

        let filter = FetchMetricLatestFilter {
            id: uuid_0,
        };
        let input = FetchMetricsLatestReq(filter);

        let res = cache.fetch(input).await;
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res.0.timestamp, 5u128);
        assert_eq!(res.0.value, 6u128);
    }

    // This is not the id you are searching for
    #[tokio::test]
    async fn fetch_02() {
        let uuid_0 = Uuid::new_v4();
        let cache = MetricHistoryCache(RwLock::new(HashMap::new()));

        let filter = FetchMetricLatestFilter {
            id: uuid_0,
        };
        let input = FetchMetricsLatestReq(filter);

        let res = cache.fetch(input).await;
        assert!(res.is_err());
    }
}
