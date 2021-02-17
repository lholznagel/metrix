mod fetch_history;
mod fetch_last;
mod fetch_last_bulk;
mod insert;
mod storage;

pub use self::fetch_history::*;
pub use self::fetch_last::*;
pub use self::fetch_last_bulk::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use std::{collections::HashMap};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Default)]
pub struct MetricHistoryCache(pub(crate) RwLock<HashMap<Uuid, Vec<MetricHistoryEntry>>>);

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Parse)]
pub struct MetricHistoryEntry {
    /// Timestamp in nano seconds
    pub timestamp: u64,
    /// Value for that specific timestamp
    pub value: u128
}
