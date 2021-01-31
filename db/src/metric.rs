use crate::{Actions, Caches, EmptyResponse, Resolve, parser_request};

use async_trait::*;
use cachem_utils::{CachemError, Parse};
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Stores the information about a metric
pub struct MetricInfo(RwLock<HashMap<Uuid, MetricEntry>>);

#[derive(Parse)]
pub struct MetricEntry {
    pub id: Uuid,
    /// For example `my_project::submodule::metric`
    pub key: String,
    pub history: Vec<HistoryEntry>,
}

impl MetricEntry {
    pub const MAX_HISTORY_COUNT: usize = 10_000;

    pub fn new(key: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            key,
            history: Vec::with_capacity(Self::MAX_HISTORY_COUNT),
        }
    }
}

#[derive(Parse)]
pub struct HistoryEntry {
    pub timestamp: u128,
    pub value: u128
}

/// Resolves the given name to a uuid
///
/// If the name is not in the cache it will be added and the uuid will be returned
#[async_trait]
impl Resolve<ResolveMetricUuid> for MetricInfo {
    type Error = EmptyResponse;
    type Response = Uuid;
    async fn resolve(&self, input: ResolveMetricUuid) -> Result<Self::Response, Self::Error> {
        let key = input.0;
        let id = self.0
            .read()
            .await
            .iter()
            .find(|(_, e)| e.key == key)
            .map(|(_, e)| e.id);

        if let Some(id) = id {
            Ok(id)
        } else {
            let entry = MetricEntry::new(key);
            let id = entry.id;
            self.0
                .write()
                .await
                .insert(entry.id, entry);
            Ok(id)
        }
    }
}

#[derive(Debug, Parse)]
pub struct ResolveMetricUuid(pub String);
parser_request!(Actions::Resolve, Caches::MetricInfo, ResolveMetricUuid);

#[derive(Debug, Parse)]
pub struct ResolveMetricUuidResponse(pub Vec<Uuid>);

pub enum Datatypes {
    /// A counter can only count up or be reset
    Counter,
    /// Duration between two actions
    Duration,
    /// A simple number that can be increased or decreased
    Simple,
}

impl From<u8> for Datatypes {
    fn from(x: u8) -> Self {
        match x {
            0u8 => Datatypes::Counter,
            1u8 => Datatypes::Duration,
            2u8 => Datatypes::Simple,
            _   => panic!("Invalid datatype {}", x)
        }
    }
}

impl From<&Datatypes> for u8 {
    fn from(x: &Datatypes) -> Self {
        match x {
            Datatypes::Counter  => 0u8,
            Datatypes::Duration => 1u8,
            Datatypes::Simple   => 2u8,
        }
    }
}

#[async_trait]
impl Parse for Datatypes {
    async fn read<B>(
        buf: &mut B
    ) -> Result<Self, CachemError>
        where
            B: AsyncBufRead + AsyncRead + Send + Unpin {

        let x = buf.read_u8().await?;
        Ok(Self::from(x))
    }

    async fn write<B>(
        &self,
        buf: &mut B
    ) -> Result<(), CachemError>
        where
            B: AsyncWrite + Send + Unpin {
        
        buf.write_u8(u8::from(self)).await?;
        Ok(())
    }
}
