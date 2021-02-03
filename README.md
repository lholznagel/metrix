# Metrix

`Metrix` is a simple implementation for metrics that exports its metrics as the prometheus exposition format.

Not all prometheus types are implemented and the current `Metric` struct does not implement all `MetricCommand` commands.

## Usage

Run:
```
git submodule add git@github.com:lholznagel/metrix.git
```

Cargo.toml:
```
metrix = { path = "../metrix" }
tokio = { version = "0.2.24", features = ["full"] }
```

Code:
``` rust
use metrix::{MetricCollector, MetricCommand, Metrics};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (metric_tx, metric_rx) = mpsc::channel::<(&str, MetricCommand)>(64);
    let metric_collector = MetricCollector::default();

    let metric_tx_copy = metric_tx.clone();
    let my_task = tokio::task::spawn(async {
        let mut metrics = Metrics::new(metric_tx_copy);

        let start = std::time::Instant::now();
        // automatically takes the elapsed time
        metrics.duration("my_metric_name", start).await;

        metrics.set("another_metric", 5u64).await;

        metrics.current_timestamp("current_timestamp").await;
    });

    tokio::join!(
        my_task,
        metric_collector.metric_server("127.0.0.1:9100"),
        metric_collector.metric_listener(metric_rx),
    );
}
```

## Defaults

- Setting a duration boils down to a prometheus summary type
  - Quantiles: 0.95, 0.99 and 1
  - To add more quantiles go to [collector.rs](./src/collector.rs#158)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
