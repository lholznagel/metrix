mod metric;

pub use self::metric::*;

use cachem_utils::Parse;

#[macro_export]
macro_rules! parser_request {
    ($action:expr, $cache:expr, $struct:ident) => {
        #[async_trait::async_trait]
        impl cachem_utils::ProtocolRequest for $struct {
            fn action(&self) -> u8 {
                $action.into()
            }

            fn cache_type(&self) -> u8 {
                $cache.into()
            }
        }
    };
}

#[derive(Debug, Default, Parse)]
pub struct EmptyResponse;

#[async_trait::async_trait]
pub trait Resolve<T: Parse> {
    type Error;
    type Response;
    async fn resolve(&self, input: T) -> Result<Self::Response, Self::Error>;
}

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
    MetricInfo,
}

impl Into<u8> for Caches {
    fn into(self) -> u8 {
        match self {
            Self::MetricInfo =>  0u8,
        }
    }
}

impl From<u8> for Caches {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::MetricInfo,
            _ => panic!("Unrecognized cache type {}", x),
        }
    }
}
