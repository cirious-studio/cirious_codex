//! # Full Ecosystem: Cirious Codex
//!
//! This example demonstrates how all pieces of the Cirious ecosystem work
//! together in a real-world scenario:
//! 1. Custom structured logging via dispatchers.
//! 2. Advanced configuration merging from multiple sources.
//! 3. Standardized error handling and result wrapping.
//!
//! You can run this example with:
//! `cargo run --example 04_full_ecosystem --all-features`

use cirious_codex::codex_config::{ConfigBuilder, Deserialize};
use cirious_codex::codex_logger::{Dispatcher, Level, Record, StdoutDispatcher, StyledTerminalFormatter};
use cirious_codex_config::format::ConfigFormat;
use cirious_codex_result::{codex_ok, CodexError, Result};
use std::env;

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
  host: String,
  port: u16,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
  worker_threads: usize,
  database: DatabaseConfig,
}

/// Helper function to manually dispatch logs, since the global macros
/// are currently lightweight placeholders in the foundational library.
fn log(level: Level, message: std::fmt::Arguments) {
  let dispatcher = StdoutDispatcher::new(StyledTerminalFormatter);
  let record = Record { level, args: message };
  dispatcher.dispatch(&record);
}

/// A simulated business logic function that uses the entire ecosystem.
/// By returning `Result<()>`, we fully leverage the `cirious_codex_result` wrapper.
fn run_application(simulate_failure: bool) -> Result<()> {
  log(Level::Info, format_args!("Bootstrapping the Cirious ecosystem..."));

  // 1. Setup the configuration base (simulating a config.json file)
  let base_config = r#"{
    "worker_threads": 4,
    "database": {
      "host": "localhost",
      "port": 5432
    }
  }"#;

  // Simulate environment variables being passed by Docker/Kubernetes
  env::set_var("SERVER_WORKER_THREADS", "16");

  log(Level::Debug, format_args!("Loading configuration sources..."));

  // 2. Build configuration safely using ConfigBuilder
  let config = ConfigBuilder::new()
    .add_source(base_config, ConfigFormat::Json)?
    .value
    .add_env_prefix("SERVER_")
    .build::<ServerConfig>()?
    .value;

  log(
    Level::Info,
    format_args!(
      "Configuration loaded successfully. Threads: {}, DB: {}:{}",
      config.worker_threads, config.database.host, config.database.port
    ),
  );

  // 3. Simulate an error to demonstrate CodexError propagation
  if simulate_failure {
    log(
      Level::Warn,
      format_args!("Simulating a critical application failure..."),
    );

    // We use the CodexError builder to construct a detailed, diagnostic-rich error
    return Err(
      CodexError::builder("DB_CONN_ERR", "Failed to establish a connection with the database")
        .with_suggestion("Check if the database host is reachable and the credentials are correct."),
    );
  }

  log(
    Level::Info,
    format_args!("Application executed successfully with no errors."),
  );

  // 4. Return success natively using the codex_ok! macro
  codex_ok!(())
}

fn main() {
  // The main function acts solely as the bridge to the Operating System.
  // We use the `run()` architectural pattern to allow our internal logic
  // to cleanly return our custom `Result<()>`.

  let simulate_failure = false; // Change to `true` to test the error path

  if let Err(e) = run_application(simulate_failure) {
    log(
      Level::Error,
      format_args!("Application crashed with fatal error:\n{:#?}", e),
    );
    std::process::exit(1);
  }
}
