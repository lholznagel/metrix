use cachem::*;
use metrix_db::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let metric_info_cache = Arc::new(MetricInfoCache::load_from_file().await?);
    let metric_history_cache = Arc::new(MetricHistoryCache::load_from_file().await?);

    cachem! {
        "0.0.0.0:8888",

        let metric_info_copy = metric_info_cache.clone();
        let metric_history_copy= metric_history_cache.clone();

        - Actions::LookupMetricId => (metric_info_copy, lookup, LookupMetricIdsReq),
        - Actions::FetchAllInfos  => (metric_info_copy, fetch, FetchAllMetricInfosReq),

        - Actions::FetchHistory   => (metric_history_copy, fetch, FetchMetricsHistoryReq),
        - Actions::FetchLast      => (metric_history_copy, fetch, FetchMetricsLastReq),
        - Actions::FetchLastBulk  => (metric_history_copy, fetch, FetchMetricsLastBulkReq),
        - Actions::InsertMetric   => (metric_history_copy, insert, InsertMetricsReq),
    };
}
