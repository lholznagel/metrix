use super::{MetricHistoryCache, MetricHistoryEntry};

use crate::Actions;

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Fetch<FetchMetricsHistoryReq> for MetricHistoryCache {
    type Response = FetchMetricsHistoryRes;

    async fn fetch(&self, filter: FetchMetricsHistoryReq) -> Self::Response {
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
            return FetchMetricsHistoryRes::Ok(res);
        }

        FetchMetricsHistoryRes::Err(EmptyMsg::default())
    }
}

#[request(Actions::FetchHistory)]
#[derive(Debug, Parse)]
pub struct FetchMetricsHistoryReq {
    /// Id of the requested metric
    pub id: Uuid,
    /// Start timestamp, max 30 days in the past
    /// Everything that exceedes 30 days is not guaranteed to be in the database
    pub ts_start: u64,
}

#[derive(Debug, Parse)]
pub enum FetchMetricsHistoryRes {
    Ok(Vec<MetricHistoryEntry>),
    Err(EmptyMsg)
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

        let input = FetchMetricsHistoryReq {
            id: uuid_0,
            ts_start: 0
        };

        match cache.fetch(input).await {
            FetchMetricsHistoryRes::Ok(x) => {
                assert!(x.len() > 0);
                assert!(x.len() == 4);
            },
            _ => assert!(false)
        }
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

        let input = FetchMetricsHistoryReq {
            id: uuid_0,
            ts_start: 3
        };

        match cache.fetch(input).await {
            FetchMetricsHistoryRes::Ok(x) => {
                assert!(x.len() > 0);
                assert!(x.len() == 2);
            },
            _ => assert!(false)
        }
    }

    // This is not the id you are searching for
    #[tokio::test]
    async fn fetch_03() {
        let uuid_0 = Uuid::new_v4();
        let cache = MetricHistoryCache(RwLock::new(HashMap::new()));

        let input = FetchMetricsHistoryReq {
            id: uuid_0,
            ts_start: 0u64,
        };

        match cache.fetch(input).await {
            FetchMetricsHistoryRes::Err(_) => assert!(true),
            _ => assert!(false)
        }
    }
}
