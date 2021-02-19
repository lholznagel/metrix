mod error;

use self::error::*;

use cachem::{ConnectionPool, EmptyResponse, Protocol};
use metrix_db::{InsertMetricsEntry, InsertMetricsReq, LookupMetricIdReq, LookupMetricIdRes};
use mpsc::{Receiver, Sender};
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Manages mspc and sending metrix to the database
pub struct Metrix {
    /// Contains all ids that are resolved
    ids: HashMap<String, Uuid>,
    /// Connection pool to the metrix database
    metrix_pool: ConnectionPool,
    /// MSPC Receiver
    receiver: Receiver<(&'static str, u128)>,
    /// MSPC Sender
    sender: Sender<(&'static str, u128)>,
}

impl Metrix {
    /// Creates a new metrix instance
    ///
    /// Takes the uri to the metrix database
    pub async fn new(metrix_db_uri: &'static str) -> Result<Self, MetrixError> {
        let (tx, rx) = mpsc::channel(1_000);

        Ok(Self {
            sender: tx,
            receiver: rx,
            ids: HashMap::new(),
            metrix_pool: ConnectionPool::new(metrix_db_uri, 50).await?,
        })
    }

    /// Gets a clone of the sender
    pub fn get_sender(&self) -> MetrixSender {
        MetrixSender(self.sender.clone())
    }

    pub async fn listen(mut self) {
        while let Some((k, v)) = self.receiver.recv().await {
            let id = if let Some(id) = self.ids.get(&k.to_string()) {
                *id
            } else {
                self.fetch_id(k).await.unwrap()
            };

            // if the connection fails, we ignore the metric
            if let Ok(mut conn) = self.metrix_pool.acquire().await {
                if let Err(e) = Protocol::request::<_, EmptyResponse>(
                    &mut conn,
                    InsertMetricsReq(
                        InsertMetricsEntry {
                            id,
                            value: v,
                        }
                    ),
                )
                .await {
                    log::error!("Error sending metric {:?} to server. Error: {:?}", id, e);
                }
            }
        }
    }

    async fn fetch_id(&mut self, name: &'static str) -> Result<Uuid, MetrixError> {
        let mut conn = self.metrix_pool.acquire().await?;
        let result = Protocol::request::<_, LookupMetricIdRes>(
            &mut conn,
            LookupMetricIdReq(name.into()),
        )
        .await
        .map(|x| x.0)?;
        self.ids.insert(result.key, result.id);
        Ok(result.id)
    }
}

#[derive(Clone)]
pub struct MetrixSender(Sender<(&'static str, u128)>);

impl MetrixSender {
    pub async fn send(&self, metric: &'static str, value: u128) {
        let _ = self.0.send((metric, value)).await.unwrap();
    }
}

// metrix!(self, METRIC, VALUE);
#[macro_export]
macro_rules! metrix {
    ($self:expr, $metric:expr, $value:expr) => {
        $self.metrix
            .as_ref()
            .unwrap()
            .send($metric, $value)
            .await;
    };
}
