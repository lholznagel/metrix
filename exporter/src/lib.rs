use reqwest::Client;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

/// Manages mspc and sending metrix to the database
pub struct Metrix {
    /// Contains all ids that are resolved
    ids: HashMap<String, Uuid>,
    /// MSPC Receiver
    receiver: Receiver<(String, u128)>,
    /// MSPC Sender
    sender: Sender<(String, u128)>,
    /// Most times this will be the crate name
    root_metric: String,
    /// Uri to metrix server
    metrix_uri: &'static str,
    /// Reqwest client
    metrix_client: Client,
}

impl Metrix {
    /// Creates a new metrix instance
    ///
    /// Takes the uri to the metrix database
    pub async fn new(root_metric: String, metrix_uri: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel(1_000_000);

        Ok(Self {
            sender: tx,
            receiver: rx,
            ids: HashMap::new(),
            metrix_uri,
            metrix_client: Client::new(),
            root_metric,
        })
    }

    /// Gets a clone of the sender
    pub fn get_sender(&self) -> MetrixSender {
        MetrixSender::new(self.root_metric.clone(), self.sender.clone())
    }

    pub async fn listen(mut self) {
        while let Some((k, v)) = self.receiver.recv().await {
            let id = if let Some(id) = self.ids.get(&k.to_string()) {
                *id
            } else {
                self.fetch_id(&k).await.unwrap()
            };

            if let Ok(r) = self.metrix_client
                .post(&format!("http://{}/api/metrics/{}", self.metrix_uri, id))
                .json(&v)
                .send()
                .await {

                if let Err(e) = r.text().await {
                    log::error!("Error sending metric {:?} to server. Error: {:?}", id, e);
                }
            } else {
                log::error!("Error sending metric {:?} to server.", id);
            }
        }
    }

    async fn fetch_id(&mut self, name: &str) -> Result<Uuid, Box<dyn std::error::Error>> {
        let id = self.metrix_client
            .post(&format!("http://{}/api/metrics", self.metrix_uri))
            .json(&name)
            .send()
            .await?
            .json::<Uuid>()
            .await?;
        self.ids.insert(name.into(), id);
        Ok(id)
    }
}

#[derive(Clone)]
pub struct MetrixSender {
    root: String,
    sender: Sender<(String, u128)>,
}

impl MetrixSender {
    fn new(root: String, sender: Sender<(String, u128)>) -> Self {
        Self {
            root,
            sender
        }
    }

    #[inline]
    pub async fn send(&self, metric: &'static str, value: u128) {
        let mut m = self.root.clone();
        m.push_str("::");
        m.push_str(metric);

        let _ = self.sender.send((m, value)).await;
    }

    #[inline]
    pub async fn send_time(&self, metric: &'static str, value: Instant) {
        let value = value.elapsed().as_nanos();
        let _ = self.send(metric, value).await;
    }

    #[inline]
    pub async fn send_len(&self, metric: &'static str, value: usize) {
        let _ = self.send(metric, value as u128).await;
    }
}
