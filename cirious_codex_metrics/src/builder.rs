use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

// Importing the exact architectural logger components from the workspace
use cirious_codex_logger::{Dispatcher, JsonFormatter, Level, Record, StdoutDispatcher, StyledTerminalFormatter};

use cirious_codex_result::{codex_ok, CodexError, Result};

/// Supported log formatting styles within the Codex ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogFormat {
  /// Structured JSON formatting optimized for cloud pipelines (Datadog, ELK, `CloudWatch`).
  Json,
  /// Beautiful ANSI color-coded formatting optimized for human readability in local terminals.
  Styled,
}

/// A fluent builder to configure and initialize global application telemetry.
///
/// This unifies both structured logging and metrics collection into a single setup flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TelemetryBuilder {
  prometheus_addr: SocketAddr,
  log_format: LogFormat,
}

impl Default for TelemetryBuilder {
  fn default() -> Self {
    Self {
      prometheus_addr: SocketAddr::from(([0, 0, 0, 0], 9000)),
      log_format: LogFormat::Styled,
    }
  }
}

impl TelemetryBuilder {
  /// Creates a new `TelemetryBuilder` instance with production-ready defaults:
  /// - Log Level: `INFO`
  /// - Prometheus Scrape Address: `0.0.0.0:9000`
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the logging format style used during telemetry status dispatches.
  #[must_use]
  pub const fn with_format(mut self, format: LogFormat) -> Self {
    self.log_format = format;
    self
  }

  /// Sets the network socket address where the Prometheus `/metrics` background HTTP listener will bind.
  #[must_use]
  pub const fn with_prometheus_addr(mut self, addr: SocketAddr) -> Self {
    self.prometheus_addr = addr;
    self
  }

  /// Consumes the builder, initializing the background Prometheus HTTP server.
  ///
  /// # Errors
  ///
  /// This method will return an error if the Prometheus recorder or the background
  /// HTTP listener fails to bind to the specified socket address (e.g., if the port is already in use).
  pub fn init(self) -> Result<()> {
    // Initialize Prometheus HTTP Exporter background listener
    PrometheusBuilder::new()
      .with_http_listener(self.prometheus_addr)
      .install()
      .map_err(|e|
        CodexError::builder("PROMETHEUS_INIT_FAILED", "Failed to install global Prometheus recorder")
          .with_suggestion("Verify if the network port is already allocated by another process or if the application lacks binding permissions.")
          .with_meta("target_address", self.prometheus_addr.to_string())
          .with_meta("underlying_error", e.to_string())
      )?;

    // 2. Format and dispatch initialization log via the chosen workspace system
    match self.log_format {
      LogFormat::Json => {
        let dispatcher = StdoutDispatcher::new(JsonFormatter);
        codex_ok!(dispatcher.dispatch(&Record {
          level: Level::Info,
          args: format_args!(
            "Prometheus metrics exporter running at http://{}/metrics",
            self.prometheus_addr
          ),
        }))
      }
      LogFormat::Styled => {
        let dispatcher = StdoutDispatcher::new(StyledTerminalFormatter);
        codex_ok!(dispatcher.dispatch(&Record {
          level: Level::Info,
          args: format_args!(
            "📊 Prometheus metrics exporter running at http://{}/metrics",
            self.prometheus_addr
          ),
        }))
      }
    }
  }
}
