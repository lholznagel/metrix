mod metric_history;
mod metric_info;

pub use self::metric_history::*;
pub use self::metric_info::*;

#[derive(Debug)]
pub enum Actions {
    FetchAll,
    FetchLatest,
    Insert,
    Lookup,
}

impl Into<u16> for Actions {
    fn into(self) -> u16 {
        match self {
            Self::FetchAll    => 0u16,
            Self::FetchLatest => 1u16,
            Self::Insert      => 2u16,
            Self::Lookup      => 3u16,
        }
    }
}

impl From<u16> for Actions {
    fn from(x: u16) -> Self {
        match x {
            0 => Actions::FetchAll,
            1 => Actions::FetchLatest,
            2 => Actions::Insert,
            3 => Actions::Lookup,
            _ => panic!("Unrecognized action {}", x),
        }
    }
}
