<div align="center">

# 🚀 Cirious Codex Metrics

**A unified telemetry and Prometheus exposition layer for robust Rust applications.**

[![CI](https://github.com/cirious-studio/cirious_codex_metrics/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex_metrics/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex_metrics.svg)](https://crates.io/crates/cirious_codex_metrics) [![Docs.rs](https://docs.rs/cirious_codex_metrics/badge.svg)](https://docs.rs/cirious_codex_metrics) [![Language](https://img.shields.io/badge/Language-Rust-black?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Cirious Codex Metrics** is a lightweight, high-performance Rust crate designed to standardize observability. By unifying `tracing` for structured logging and span execution with the `metrics` ecosystem for Prometheus exposition, it provides developers with a frictionless, drop-in path to production-grade telemetry. 

Instead of configuring multiple overlapping libraries in every new project, `cirious_codex_metrics` offers a single initialization point to get your logs formatted and your Prometheus `/metrics` endpoint serving immediately.

---

## 📋 Quick Start

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_metrics = "0.1.0"
```

### Basic Usages

```rust
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

``` 

---

## 🚧 Current Status & Roadmap

### ✅ v0.1.0 — Initial Release (Completed)

- [x] **Ecosystem Logger Integration:** Intercept lifecycle state using native codex_logger dispatchers, eliminating redundant external dependencies.
- [x] **Ecosystem Result Integration:** Use `cirious_codex_result` for structured error handling and result types.
- [x] **Format Profile Support:** Native compliance with JsonFormatter and StyledTerminalFormatter for environmental log layout switching.
- [x] **Prometheus Exporter Setup:** Integrate `metrics-exporter-prometheus` to automatically manage registries and formatting.
- [x] **Standalone HTTP Endpoint:** Spin up a lightweight, background HTTP listener dedicated to serving the `/metrics` scrape target.
- [x] **Ergonomic Facade Re-exports:** Module isolation for `metrics_registry` and `tokio` to keep user dependencies clean.
- [x] **Documentation & Examples:** Write comprehensive doc comments and provide runnable workspace integration examples.
- [x] **Publish:** push `cirious_codex_metrics` to `crates.io` as a standalone crate.

### 🚀 v0.2.0 — Production Readiness & Integrations (Planned)

- [ ] **Ecosystem Error Handling Integration:** Transition from raw panic statements on HTTP socket binding failures to structured diagnostic returns leveraging `cirious_codex_result::CodexError`.
- [ ] **HTTP Middleware (Tower/Axum):** Deliver plug-and-play middleware configurations to systematically track server request rates, routing latencies, and error counters.
- [ ] **Host System Metrics:** Implement automated background loop scraping for runtime system parameters (CPU cycles, memory allocation, and OS threads).

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by Cirious Studio</sub>
</div>
