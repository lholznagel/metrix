mod fetch_all;
mod lookup;
mod storage;

pub use self::fetch_all::*;
pub use self::lookup::*;
pub use self::storage::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Default)]
pub struct MetricInfoCache(RwLock<HashMap<Uuid, MetricInfoEntry>>);

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Parse)]
pub struct MetricInfoEntry {
    /// Id of the metric
    pub id: Uuid,
    /// For example `my_project::submodule::metric`
    pub key: String,
}

impl MetricInfoEntry {
    pub fn new(key: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            key,
        }
    }
}
