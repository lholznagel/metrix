use super::{MetricHistoryCache, MetricHistoryEntry};

use async_trait::*;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};
use tokio::sync::RwLock;
use uuid::Uuid;

#[async_trait]
impl Storage for MetricHistoryCache {
    fn file() -> &'static str {
        "./db/storage/metric_history.cachem"
    }

    async fn load<B>(buf: &mut B) -> Result<Self, CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        if let Ok(entries) = SaveMetrics::read(buf).await {
            let mut map = HashMap::with_capacity(entries.0.len());

            for entry in entries.0 {
                map.insert(entry.id, entry.entries);
            }

            Ok(Self(RwLock::new(map)))
        } else {
            Ok(Self::default())
        }
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let data_copy = self.0.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (id, entries) in data_copy.iter() {
            let entry = SaveMetricEntry {
                id: *id,
                entries: entries.clone(),
            };
            save_entries.push(entry);
        }

        SaveMetrics(save_entries)
            .write(buf)
            .await
            .map(drop)
    }
}

#[derive(Debug, Parse)]
pub struct SaveMetrics(pub Vec<SaveMetricEntry>);

#[derive(Debug, Parse)]
pub struct SaveMetricEntry {
    /// Id of the metric
    pub id: Uuid,
    /// All entries for that metric
    pub entries: Vec<MetricHistoryEntry>,
}
