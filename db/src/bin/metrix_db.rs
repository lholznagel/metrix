use cachem::*;
use metrix_db::*;
use tokio::signal::unix::{SignalKind, signal};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let metric_info_cache = MetricInfoCache::default();
    metric_info_cache.load_from_file().await?;
    let metric_info_cache = Arc::new(metric_info_cache);

    let metric_history_cache = MetricHistoryCache::default();
    metric_history_cache.load_from_file().await?;
    let metric_history_cache = Arc::new(metric_history_cache);

    let metric_info_copy = metric_info_cache.clone();
    let metric_history_copy = metric_history_cache.clone();
    tokio::task::spawn(async move {
        let mut stream = signal(SignalKind::terminate()).unwrap();
        loop {
            stream.recv().await;
            println!("Got SIGTERM, saving.");
            let _ = metric_info_copy.save_to_file().await;
            let _ = metric_history_copy.save_to_file().await;
            std::process::exit(0);
        }
    });

    cachem! {
        "0.0.0.0:8888",

        let metric_info_copy = metric_info_cache.clone();
        let metric_history_copy= metric_history_cache.clone();

        - Actions::LookupMetricId => (metric_info_copy, lookup, LookupMetricIdReq),
        - Actions::FetchAllInfos  => (metric_info_copy, fetch, FetchAllMetricInfosReq),

        - Actions::FetchHistory   => (metric_history_copy, fetch, FetchMetricsHistoryReq),
        - Actions::FetchLast      => (metric_history_copy, fetch, FetchMetricsLastReq),
        - Actions::FetchLastBulk  => (metric_history_copy, fetch, FetchMetricsLastBulkReq),
        - Actions::InsertMetric   => (metric_history_copy, insert, InsertMetricsReq),
    };
}
