use super::MetricHistoryCache;

use crate::Actions;

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};
use uuid::Uuid;

#[async_trait]
impl Fetch<FetchMetricsLastBulkReq> for MetricHistoryCache {
    type Error    = EmptyResponse;
    type Response = FetchMetricsLastBulkRes;

    async fn fetch(&self, input: FetchMetricsLastBulkReq) -> Result<Self::Response, Self::Error> {
        let mut result = Vec::with_capacity(input.0.len());

        let entries = self.0
            .read()
            .await;
        for id in input.0 {
            if let Some(x) = entries.get(&id) {
                // If the item exist, there is at least one element in the vec
                let entry = x.clone().pop().unwrap();
                result.push(MetricHistoryBulkEntry {
                    id,
                    timestamp: entry.timestamp,
                    value: entry.value,
                });
            }
        }

        Ok(FetchMetricsLastBulkRes(result))
    }
}

#[request(Actions::FetchLastBulk)]
#[derive(Debug, Parse)]
pub struct FetchMetricsLastBulkReq(pub Vec<Uuid>);

#[derive(Debug, Parse)]
pub struct FetchMetricsLastBulkRes(pub Vec<MetricHistoryBulkEntry>);

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Parse)]
pub struct MetricHistoryBulkEntry {
    /// Id of the metric
    pub id: Uuid,
    /// Timestamp in nano seconds
    pub timestamp: u64,
    /// Value for that specific timestamp
    pub value: u128
}
