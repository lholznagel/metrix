use super::{MetricInfoCache, MetricInfoEntry};

use async_trait::*;
use cachem::{CachemError, Parse, Storage};
use std::collections::HashMap;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};
use tokio::sync::RwLock;

#[async_trait]
impl Storage for MetricInfoCache {
    fn file() -> &'static str {
        "./db/storage/metric_info.cachem"
    }

    async fn load<B>(buf: &mut B) -> Result<Self, CachemError>
        where B: AsyncBufRead + AsyncRead + Send + Unpin {

        let entries = SaveMetrics::read(buf).await?;
        let mut map = HashMap::with_capacity(entries.0.len());
        for entry in entries.0 {
            map.insert(entry.id, entry);
        }

        Ok(MetricInfoCache(RwLock::new(map)))
    }

    async fn save<B>(&self, buf: &mut B) -> Result<(), CachemError>
        where B: AsyncWrite + Send + Unpin {

        let data_copy = self.0.read().await;

        let mut save_entries = Vec::with_capacity(data_copy.len());
        for (_, entry) in data_copy.iter() {
            save_entries.push(entry.clone());
        }

        SaveMetrics(save_entries)
            .write(buf)
            .await
            .map(drop)
    }
}

#[derive(Debug, Parse)]
pub struct SaveMetrics(pub Vec<MetricInfoEntry>);
