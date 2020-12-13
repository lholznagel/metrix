use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::{Mutex, mpsc::Receiver}};
use tokio::prelude::*;

use crate::{MetricCommand, MetricData};

/// Handles the collecting and exporting of application metrics
#[derive(Default)]
pub struct MetricCollector {
    metrics: Arc<Mutex<HashMap<&'static str, MetricData>>>,
}

impl MetricCollector {
    /// Listens for new metrics over the mspc receiver channel
    /// This function is blocking!
    pub async fn metric_listener(&self, rx: Receiver<(&'static str, MetricCommand)>) {
        let mut rx = rx;
        while let Some((k, m)) = rx.recv().await {
            match m {
                MetricCommand::Counter => {
                    self.metrics
                        .lock()
                        .await
                        .entry(k)
                        .and_modify(|x| x.increase(1))
                        .or_insert(MetricData::Counter(1));
                }
                MetricCommand::CurrentTimestamp => {
                    self.metrics
                        .lock()
                        .await
                        .entry(k)
                        .and_modify(|x| x.current_timestamp())
                        .or_insert(MetricData::new_current_timestamp());
                }
                MetricCommand::Decrease(v) => {
                    self.metrics
                        .lock()
                        .await
                        .entry(k)
                        .and_modify(|x| x.decrease(v))
                        .or_insert(MetricData::Gauge(v));
                }
                MetricCommand::Duration(v) => {
                    self.metrics
                        .lock()
                        .await
                        .entry(k)
                        .and_modify(|x| x.duration(v))
                        .or_insert(MetricData::Duration(vec![v]));
                }
                MetricCommand::Increase(v) => {
                    self.metrics
                        .lock()
                        .await
                        .entry(k)
                        .and_modify(|x| x.increase(v))
                        .or_insert(MetricData::Gauge(v));
                }
                MetricCommand::Set(v) => {
                    self.metrics
                        .lock()
                        .await
                        .entry(k)
                        .and_modify(|x| x.set(v))
                        .or_insert(MetricData::Gauge(v));
                }
            }
        }
    }

    /// Generates the prometheus exposition format and returns it
    pub async fn export(&self) -> Vec<u8> {
        let metrics = self.metrics.lock().await.clone();
        crate::exporters::prometheus_exposition(metrics)
    }

    /// Starts a tcp listener and returns the prometheus exposition format when a message is send
    pub async fn metric_server(&self, addr: &'static str) {
        let mut listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let metrics = self.metrics.lock().await.clone();
            let output = crate::exporters::prometheus_exposition(metrics);

            tokio::spawn(async move {
                // seems like we have to read before writing if not we get an "Connection reset by peer"
                // cUrl and firefox are ok with 512 bytes but Chrome needs some more just in case we read 1024 bytes.
                let mut buf = [0; 1024];

                match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(_) => return
                };

                let mut response = "HTTP/1.1 200\r\n\r\n".as_bytes().to_vec();
                response.extend(&output);

                // Write the data back
                if let Err(e) = socket.write_all(&response).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            });
        }
    }
}