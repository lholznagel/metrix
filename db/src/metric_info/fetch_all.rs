use crate::{Actions, MetricInfoCache, MetricInfoEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchAllMetricInfosReq> for MetricInfoCache {
    type Response = FetchAllMetricInfosRes;

    async fn fetch(&self, _: FetchAllMetricInfosReq) -> Self::Response {
        let entries = self.0
            .read()
            .await
            .clone()
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        FetchAllMetricInfosRes(entries)
    }
}

#[request(Actions::FetchAllInfos)]
#[derive(Debug, Default, Parse)]
pub struct FetchAllMetricInfosReq(pub EmptyMsg);

#[derive(Debug, Parse)]
pub struct FetchAllMetricInfosRes(pub Vec<MetricInfoEntry>);
