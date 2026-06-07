//! # Metrics with Cirious Codex
//!
//! This example demonstrates how to build a metrics where the application
//! using `cirious_codex_metrics`.
//!
//! You can run this example with:
//! `cargo run --example 05_metrics_build --all-features`

use cirious_codex::codex_metrics::{metrics_registry::counter, tokio, LogFormat, TelemetryBuilder};
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = SocketAddr::from(([127, 0, 0, 1], 9100));

  // Bootstrap using the Styled Terminal profile (Great for Dev environments)
  TelemetryBuilder::new()
    .with_prometheus_addr(addr)
    .with_format(LogFormat::Styled)
    .init()?;

  loop {
    counter!("cirious_processed_ticks_total").increment(1);
    tokio::time::sleep(Duration::from_secs(1)).await;
  }
}
