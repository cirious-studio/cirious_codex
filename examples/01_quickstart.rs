//! # Quickstart: Cirious Codex
//!
//! This example demonstrates the basic initialization of the Cirious ecosystem,
//! combining the logger, configuration parsing, and error handling.
//!
//! You can run this example with:
//! `cargo run --example 01_quickstart --all-features`

use cirious_codex::codex_config::{ConfigBuilder, Deserialize};
use cirious_codex::codex_logger;
use cirious_codex_config::format::ConfigFormat;
use cirious_codex_result::CodexError;

#[derive(Debug, Deserialize)]
struct AppConfig {
  host: String,
  port: u16,
}

fn main() -> Result<(), CodexError> {
  codex_logger::info!("Starting the system...");

  let content = r#"
        (
            host: "127.0.0.1",
            port: 8080,
        )
    "#;

  let config = ConfigBuilder::new()
    .add_source(content, ConfigFormat::Ron)?
    .value
    .build::<AppConfig>()?;

  codex_logger::info!("Configuration loaded: {}:{}", config.value.host, config.value.port);

  Ok(())
}
