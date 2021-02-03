mod metric_history;
mod metric_info;

pub use self::metric_history::*;
pub use self::metric_info::*;

use cachem::Parse;

#[derive(Debug, Default, Parse)]
pub struct EmptyResponse;

#[derive(Debug)]
pub enum Actions {
    Fetch,
    Insert,
    Lookup,
    Resolve,
}

impl Into<u8> for Actions {
    fn into(self) -> u8 {
        match self {
            Self::Fetch   => 0u8,
            Self::Insert  => 1u8,
            Self::Lookup  => 2u8,
            Self::Resolve => 3u8,
        }
    }
}

impl From<u8> for Actions {
    fn from(x: u8) -> Self {
        match x {
            0 => Actions::Fetch,
            1 => Actions::Insert,
            2 => Actions::Lookup,
            3 => Actions::Resolve,
            _ => panic!("Unrecognized action {}", x),
        }
    }
}

#[derive(Debug)]
pub enum Caches {
    MetricHistory,
    MetricInfo,
}

impl Into<u8> for Caches {
    fn into(self) -> u8 {
        match self {
            Self::MetricHistory => 0u8,
            Self::MetricInfo    => 1u8,
        }
    }
}

impl From<u8> for Caches {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::MetricHistory,
            1 => Self::MetricInfo,
            _ => panic!("Unrecognized cache type {}", x),
        }
    }
}
