# Telemetry & Prometheus Exposition

The `cirious_codex_metrics` crate fundamentally simplifies application observability by providing a frictionless, native Prometheus exporter fully integrated with the Cirious ecosystem's logging and diagnostic tools.

---

## TelemetryBuilder

Instead of manually configuring overlapping metric registries, background HTTP servers, and initialization logs, you configure your observability layer through the `TelemetryBuilder`. 

When you consume the builder via `init()`, it automatically:
1. Spawns a background **Prometheus HTTP listener** on your specified socket address.
2. Dispatches a unified startup log using the native ecosystem dispatchers (e.g., `JsonFormatter` or `StyledTerminalFormatter`), ensuring your metrics engine speaks the exact same visual language as your application logs.

---

### Configuring the Exporter

You can chain builder methods to define your scrape target and environmental log profile:

```rust
use cirious_codex::codex_metrics::{LogFormat, TelemetryBuilder};
use std::net::SocketAddr;

fn setup_telemetry() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9100));

    TelemetryBuilder::new()
        .with_prometheus_addr(addr)
        // Switch to LogFormat::Json for production environments
        .with_format(LogFormat::Styled) 
        .init()?; // Returns a CodexError if the port is already in use
    
    Ok(())
}
```
---

# Recording Metrics
Once the exporter is initialized, registering and updating metrics across your application is completely decoupled. The crate provides the metrics_registry facade, so you never have to manage third-party dependencies directly in your downstream crates.

Using the ergonomic macros, you can record counters, gauges, and histograms anywhere in your codebase:

```rust
use cirious_codex_metrics::{metrics_registry::counter, tokio};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize telemetry...
    setup_telemetry()?;

    // 2. Record metrics seamlessly in your application loops
    loop {
        counter!("cirious_processed_events_total").increment(1);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
```

---

*Note: The init() method returns a standard ecosystem Result<()>. If the Prometheus server fails to bind (e.g., if the network port is already allocated by another process), it returns a fully populated CodexError containing the underlying OS failure, target address metadata, and actionable suggestions.*
