# Metrix

## Usage Metrix exporter

``` toml
# Cargo.toml

metrix_exporter = { git = "https://github.com/lholznagel/metrix.git", rev = "LATEST_REF" }
```

``` rust
// Create a new metrix instance
let mut metrix = Metrix::new("0.0.0.0:8888").await.unwrap();

// Get a metric sender
let sender = metrix.get_sender();

// Start the metric receiver as a task
tokio::task::spawn(async move {
    metrix.listen().await
});

// Send values
sender.send("my::cool::metric", 1u128).await;
```

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
