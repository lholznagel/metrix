mod lookup;
mod storage;

pub use self::lookup::*;
pub use self::storage::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Default)]
pub struct MetricInfoCache(RwLock<HashMap<Uuid, MetricInfoEntry>>);

#[derive(Clone, Debug, Parse)]
pub struct MetricInfoEntry {
    /// Id of the metric
    pub id: Uuid,
    /// For example `my_project::submodule::metric`
    pub key: String,
    /// Ids this metric is connected to
    pub links: Vec<Uuid>
}

impl MetricInfoEntry {
    pub fn new(key: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            key,
            links: Vec::new()
        }
    }
}
