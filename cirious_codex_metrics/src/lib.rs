//! # Cirious Codex Metrics
//!
//! A unified telemetry and Prometheus exposition layer built natively for the Cirious ecosystem.

/// Builder module for configuring telemetry.
pub mod builder;

/// Re-exported `metrics` crate facade.
///
/// Downstream applications can use this to register counters, gauges, and histograms.
pub mod metrics_registry {
  pub use metrics::*;
}

/// Re-exported `tokio` crate facade.
///
/// Downstream applications can use this for async operations and timers.
pub mod tokio {
  pub use tokio::*;
}

pub use builder::{LogFormat, TelemetryBuilder};
