use super::{MetricHistoryCache, MetricHistoryEntry};

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
                result.push(x.clone().pop().unwrap());
            }
        }

        Ok(FetchMetricsLastBulkRes(result))
    }
}

#[request(Actions::FetchLastBulk)]
#[derive(Debug, Parse)]
pub struct FetchMetricsLastBulkReq(pub Vec<Uuid>);

#[derive(Debug, Parse)]
pub struct FetchMetricsLastBulkRes(pub Vec<MetricHistoryEntry>);
