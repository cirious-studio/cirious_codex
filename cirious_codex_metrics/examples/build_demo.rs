//! Example of using `cirious_codex_metrics` crate

use cirious_codex_metrics::{metrics_registry::counter, tokio, LogFormat, TelemetryBuilder};
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = SocketAddr::from(([127, 0, 0, 1], 9100));

  // Example A: Bootstrap using the Styled Terminal profile (Great for Dev environments)
  TelemetryBuilder::new()
    .with_prometheus_addr(addr)
    .with_format(LogFormat::Styled)
    .init()?;

  // Example B: Switch to standard JSON output if deploying to Staging/Prod
  // TelemetryBuilder::new()
  //     .with_prometheus_addr(addr)
  //     .with_format(LogFormat::Json)
  //     .init();

  loop {
    counter!("cirious_processed_ticks_total").increment(1);
    tokio::time::sleep(Duration::from_secs(1)).await;
  }
}
