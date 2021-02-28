mod metric_history;
mod metric_info;

pub use self::metric_history::*;
pub use self::metric_info::*;

use cachem::Action;

#[derive(Debug, Action)]
pub enum Actions {
    LookupMetricId,
    FetchAllInfos,

    FetchHistory,
    FetchLast,
    FetchLastBulk,
    InsertMetric,

    Invalid,
}
