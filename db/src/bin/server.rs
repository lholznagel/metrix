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

        (Actions::Fetch, Caches::MetricInfo)     => (metric_info_copy, lookup, LookupMetricUuid),

        (Actions::Fetch, Caches::MetricHistory)  => (metric_history_copy, fetch, FetchMetricsReq),
        (Actions::Insert, Caches::MetricHistory) => (metric_history_copy, insert, InsertMetricsReq),
    };
}
