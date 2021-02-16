use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Fetch<FetchMetricsReq> for MetricHistoryCache {
    type Error    = EmptyResponse;
    type Response = FetchMetricsRes;

    async fn fetch(&self, input: FetchMetricsReq) -> Result<Self::Response, Self::Error> {
        dbg!("hjistry");
        let filter = input.0;
        let entries = self.0
            .read()
            .await;
        if let Some(e) = entries.get(&filter.id) {
            let mut res = Vec::new();
            for x in e
                .iter()
                .filter(|y| y.timestamp >= filter.ts_start) {

                res.push(*x);
            }
            return Ok(FetchMetricsRes(res));
        }

        Err(EmptyResponse::default())
    }
}

#[request(Actions::FetchAll)]
#[derive(Debug, Parse)]
pub struct FetchMetricsReq(pub FetchMetricFilter);

#[derive(Debug, Parse)]
pub struct FetchMetricsRes(pub Vec<MetricHistoryEntry>);

#[derive(Clone, Copy, Debug, Parse)]
pub struct FetchMetricFilter {
    /// Id of the requested metric
    pub id: Uuid,
    /// Start timestamp, max 30 days in the past
    /// Everything that exceedes 30 days is not guaranteed to be in the database
    pub ts_start: u64,
}

#[cfg(test)]
mod metric_history_fetch_tests {
    use super::*;

    use std::collections::HashMap;
    use tokio::sync::RwLock;

    /// Has a set of values that should all be returned
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

        let filter = FetchMetricFilter {
            id: uuid_0,
            ts_start: 0
        };
        let input = FetchMetricsReq(filter);

        let res = cache.fetch(input).await;
        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.0.len() > 0);
        assert!(res.0.len() == 4);
    }

    /// Has a set of values only 2 of 4 should be returned
    #[tokio::test]
    async fn fetch_02() {
        let uuid_0 = Uuid::new_v4();
        let mut entries = HashMap::new();
        entries.insert(uuid_0, vec![
            MetricHistoryEntry { timestamp: 0u64, value: 0u128 },
            MetricHistoryEntry { timestamp: 1u64, value: 2u128 },
            MetricHistoryEntry { timestamp: 3u64, value: 4u128 },
            MetricHistoryEntry { timestamp: 5u64, value: 6u128 },
        ]);
        let cache = MetricHistoryCache(RwLock::new(entries));

        let filter = FetchMetricFilter {
            id: uuid_0,
            ts_start: 3
        };
        let input = FetchMetricsReq(filter);

        let res = cache.fetch(input).await;
        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.0.len() > 0);
        assert!(res.0.len() == 2);
    }

    // This is not the id you are searching for
    #[tokio::test]
    async fn fetch_03() {
        let uuid_0 = Uuid::new_v4();
        let cache = MetricHistoryCache(RwLock::new(HashMap::new()));

        let filter = FetchMetricFilter {
            id: uuid_0,
            ts_start: 0u64,
        };
        let input = FetchMetricsReq(filter);

        let res = cache.fetch(input).await;
        assert!(res.is_err());
    }
}
