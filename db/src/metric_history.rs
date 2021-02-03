mod fetch;
mod fetch_latest;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::fetch_latest::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use std::{collections::HashMap};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct MetricHistoryCache(pub(crate) RwLock<HashMap<Uuid, Vec<MetricHistoryEntry>>>);

#[derive(Clone, Copy, Debug, Parse)]
pub struct MetricHistoryEntry {
    /// Timestamp in nano seconds
    pub timestamp: u128,
    /// Value for that specific timestamp
    pub value: u128
}
