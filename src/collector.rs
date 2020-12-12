use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::{Mutex, mpsc::Receiver}};
use tokio::prelude::*;

use crate::{MetricCommand, MetricData};

#[derive(Default)]
pub struct MetricCollector {
    metrics: Arc<Mutex<HashMap<&'static str, MetricData>>>,
}

impl MetricCollector {
    pub async fn background(&self, rx: Receiver<(&'static str, MetricCommand)>) {
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

    pub async fn metric_server(&self, addr: &'static str) {
        let mut listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let metrics = self.metrics.lock().await.clone();
            let output = self.output(metrics);

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

    fn output(&self, metrics: HashMap<&'static str, MetricData>) -> Vec<u8> {
        // #<space>TYPE<space>
        let type_field = "# TYPE ".as_bytes();
        let counter = " counter\n".as_bytes();
        let gauge = " gauge\n".as_bytes();
        let summary = " summary\n".as_bytes();

        let mut output = Vec::with_capacity(2048);

        for (key, val) in metrics {
            let key_bytes = key.as_bytes();
            output.extend(type_field.iter());
            output.extend(key_bytes);

            match val {
                MetricData::Counter(x) => {
                    // <sapce>counter<lf>
                    output.extend(counter);

                    output.extend(key_bytes);
                    output.push(32); // <space>
                    output.extend(x.to_string().as_bytes());
                },
                MetricData::Gauge(x) => {
                    // <sapce>gauge<lf>
                    output.extend(gauge);

                    output.extend(key_bytes);
                    output.push(32); // <space>
                    output.extend(x.to_string().as_bytes());
                },
                MetricData::Duration(x) => {
                    // <sapce>summary<lf>
                    output.extend(summary);

                    let count = x.len();
                    let quantile_95 = (0.95f32 * (count + 1) as f32) as usize - 1;
                    let quantile_99 = (0.99f32 * (count + 1) as f32) as usize - 1;

                    output.extend(key_bytes);
                    output.extend("{quantile=\"0.95\"} ".as_bytes());
                    output.extend(x.get(quantile_95).unwrap_or(&0).to_string().as_bytes());
                    output.push(10);
                    output.extend(key_bytes);
                    output.extend("{quantile=\"0.99\"} ".as_bytes());
                    output.extend(x.get(quantile_99).unwrap_or(&0).to_string().as_bytes());
                    output.push(10);
                    output.extend(key_bytes);
                    output.extend("{quantile=\"1\"} ".as_bytes());
                    output.extend(x.get(x.len() - 1).unwrap_or(&0).to_string().as_bytes());
                    output.push(10);
                    output.extend(key_bytes);
                    output.extend("_sum ".as_bytes());
                    output.extend(x.iter().sum::<u64>().to_string().as_bytes());
                    output.push(10);
                    output.extend(key_bytes);
                    output.extend("_count ".as_bytes());
                    output.extend(count.to_string().as_bytes());
                }
            }

            output.extend([10, 10].iter());
        }

        output
    }
}