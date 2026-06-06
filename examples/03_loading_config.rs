//! # Configuration Management with Cirious Codex
//!
//! This example demonstrates how to load, merge, and parse application settings
//! using multiple sources (like raw strings, files, and environment variables)
//! with `cirious_codex_config`.
//!
//! You can run this example with:
//! `cargo run --example 03_loading_config --all-features`

use cirious_codex::codex_config::{ConfigBuilder, Deserialize};
use cirious_codex::codex_logger;
use cirious_codex_config::format::ConfigFormat;
use cirious_codex_result::CodexError;
use std::env;

#[derive(Debug, Deserialize)]
struct AppSettings {
  app_name: String,
  port: u16,
  debug_mode: bool,
}

fn main() -> Result<(), CodexError> {
  // 1. Initialize the logger to observe the process
  codex_logger::info!("Starting configuration loading example...");

  // 2. Define a base configuration using JSON
  // In a real application, you would read this from a file (e.g., config.json)
  let base_json = r#"{
        "app_name": "CiriousDemo",
        "port": 8080,
        "debug_mode": false
    }"#;

  // 3. Simulate setting environment variables that override the base config
  // The ConfigBuilder will look for variables starting with "APP_"
  env::set_var("APP_PORT", "9000");
  env::set_var("APP_DEBUG_MODE", "true");

  // 4. Build the configuration by merging the sources sequentially
  // The order matters: later sources override earlier ones!
  let settings = ConfigBuilder::new()
    // Add the base JSON source
    .add_source(base_json, ConfigFormat::Json)?
    .value // Extract the builder from the CodexOk wrapper
    // Override with environment variables prefixed with "APP_"
    .add_env_prefix("APP_")
    // Build and deserialize into our strongly-typed struct
    .build::<AppSettings>()?
    .value; // Extract the final struct from the CodexOk wrapper

  // 5. Verify the merged results
  codex_logger::info!("Loaded Application Name: {}", settings.app_name);
  codex_logger::info!("Server Port: {} (Overridden by ENV variable)", settings.port);
  codex_logger::info!("Debug Mode: {} (Overridden by ENV variable)", settings.debug_mode);

  Ok(())
}
