pub mod collector;
pub mod data;
pub mod exporters;

pub use self::collector::*;
pub use self::data::*;
pub use self::exporters::*;

use std::time::Instant;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum MetricCommand {
    Counter,
    CurrentTimestamp,
    Increase(u64),
    Decrease(u64),
    Set(u64),
    Duration(u64),
}

pub struct Metrics(Sender<(&'static str, MetricCommand)>);

impl Metrics {
    pub fn new(tx: Sender<(&'static str, MetricCommand)>) -> Self {
        Self(tx)
    }

    pub async fn set(&mut self, name: &'static str, value: u64) {
        if let Err(_) = self.0.send((name, MetricCommand::Set(value))).await {
            log::error!("Error sending metric {}", name);
        }
    }

    pub async fn duration(&mut self, name: &'static str, duration: Instant) {
        let duration = duration.elapsed().as_millis() as u64;
        if let Err(_) = self.0.send((name, MetricCommand::Duration(duration))).await {
            log::error!("Error sending metric {}", name);
        }
    }

    pub async fn current_timestamp(&mut self, name: &'static str) {
        if let Err(_) = self.0.send((name, MetricCommand::CurrentTimestamp)).await {
            log::error!("Error sending metric {}", name);
        }
    }
}