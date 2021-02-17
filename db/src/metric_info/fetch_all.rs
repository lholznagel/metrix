use crate::{Actions, MetricInfoCache, MetricInfoEntry};

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchAllMetricInfosReq> for MetricInfoCache {
    type Error    = EmptyResponse;
    type Response = FetchAllMetricInfosRes;

    async fn fetch(&self, _: FetchAllMetricInfosReq) -> Result<Self::Response, Self::Error> {
        let entries = self.0
            .read()
            .await
            .clone()
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(FetchAllMetricInfosRes(entries))
    }
}

#[request(Actions::FetchAllInfos)]
#[derive(Debug, Default, Parse)]
pub struct FetchAllMetricInfosReq(pub EmptyResponse);

#[derive(Debug, Parse)]
pub struct FetchAllMetricInfosRes(pub Vec<MetricInfoEntry>);
