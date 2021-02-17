mod metric_history;
mod metric_info;

pub use self::metric_history::*;
pub use self::metric_info::*;

#[derive(Debug)]
pub enum Actions {
    LookupMetricId,
    FetchAllInfos,

    FetchHistory,
    FetchLast,
    FetchLastBulk,
    InsertMetric,
}

impl Into<u16> for Actions {
    fn into(self) -> u16 {
        match self {
            Self::LookupMetricId => 1u16,
            Self::FetchAllInfos  => 2u16,

            Self::FetchHistory   => 10u16,
            Self::FetchLast      => 11u16,
            Self::FetchLastBulk  => 12u16,
            Self::InsertMetric   => 13u16,
        }
    }
}

impl From<u16> for Actions {
    fn from(x: u16) -> Self {
        match x {
            1  => Actions::LookupMetricId,
            2  => Actions::FetchAllInfos,

            10 => Actions::FetchHistory,
            11 => Actions::FetchLast,
            12 => Actions::FetchLastBulk,
            13 => Actions::InsertMetric,
            _  => panic!("Unrecognized action {}", x),
        }
    }
}
