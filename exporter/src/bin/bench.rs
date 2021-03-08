use metrix_exporter::*;
use std::time::Instant;

const RUNS: usize = 500;
const ITERS: usize = 500;

#[tokio::main]
async fn main() {
    let metrix = Metrix::new("root".into(), "0.0.0.0:8889").await.unwrap();
    let sender = metrix.get_sender();

    tokio::task::spawn(async { metrix.listen().await; });

    let mut all_runs = Vec::new();

    for run in 0..RUNS {
        let start = Instant::now();

        for _ in 0..ITERS {
            sender.send_len("s".into(), 100usize).await;
        }

        let elapsed = start.elapsed().as_nanos();
        let avg = elapsed as f64 / ITERS as f64;
        all_runs.push(avg);
        let avg_all = all_runs.iter().sum::<f64>() / all_runs.len() as f64;
        println!("[{:5}] avg: {:10.2} ns avg all: {:10.2} ns", run, avg, avg_all);
    }
}
