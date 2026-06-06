//! # Custom Logging with Cirious Codex
//!
//! This example demonstrates how to build and dispatch custom structured
//! log records using different formatters and dispatchers provided by the
//! `cirious_codex_logger` crate.
//!
//! You can run this example with:
//! `cargo run --example 02_custom_logging --all-features`

use cirious_codex::codex_logger::{
  Dispatcher, JsonFormatter, Level, Record, StdoutDispatcher, StyledTerminalFormatter,
};

fn main() {
  // 1. Manually create a log record with detailed metadata
  // In the future, this will be automatically handled by the global macros
  let args = format_args!("User authentication failed for IP 192.168.1.10");
  let record = Record {
    level: Level::Warn,
    args,
  };

  println!("--- Structured JSON Logging ---");
  // 2. Dispatch using the structured JSON Formatter
  // Ideal for shipping logs to Datadog, ELK, or CloudWatch
  let json_dispatcher = StdoutDispatcher::new(JsonFormatter);
  json_dispatcher.dispatch(&record);

  println!("\n--- Colored Terminal Logging ---");
  // 3. Dispatch using the Styled Terminal Formatter
  // Uses `cirious_codex_term` under the hood to add beautiful ANSI colors
  let styled_dispatcher = StdoutDispatcher::new(StyledTerminalFormatter);
  styled_dispatcher.dispatch(&record);
}
