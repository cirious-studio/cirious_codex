use cirious_codex::codex_config::{ConfigBuilder, Deserialize};
use cirious_codex::codex_logger::{Dispatcher, HumanReadableFormatter, Level, Record, StdoutDispatcher};
use cirious_codex_config::format::ConfigFormat;
use cirious_codex_result::{codex_ok, CodexError, Result};

mod common;

#[derive(Debug, Deserialize, PartialEq)]
struct AppConfig {
  name: String,
  version: String,
}

fn simulate_app_startup(fail: bool) -> Result<AppConfig> {
  let dispatcher = StdoutDispatcher::new(HumanReadableFormatter);
  dispatcher.dispatch(&Record {
    level: Level::Info,
    args: format_args!("Testing ecosystem initialization"),
  });

  if fail {
    return Err(
      CodexError::builder("TEST_ERR", "Simulated failure")
        .with_suggestion("Do not pass fail=true")
        .with_meta("context", "integration_test"),
    );
  }

  let json = r#"{ "name": "CodexTest", "version": "1.0.0" }"#;

  let config = ConfigBuilder::new()
    .add_source(json, ConfigFormat::Json)?
    .value
    .build::<AppConfig>()?
    .value;

  codex_ok!(config)
}

#[test]
fn test_ecosystem_success() {
  common::setup();

  let result = simulate_app_startup(false);
  assert!(result.is_ok());

  let ok_val = result.unwrap();
  assert_eq!(ok_val.value.name, "CodexTest");
  assert_eq!(ok_val.value.version, "1.0.0");

  // Verify execution metadata is captured
  assert!(ok_val.location.file().ends_with("integration_ecosystem.rs"));
}

#[test]
fn test_ecosystem_failure() {
  common::setup();

  let result = simulate_app_startup(true);
  assert!(result.is_err());

  let err = result.unwrap_err();
  assert_eq!(err.name(), "TEST_ERR");
  assert_eq!(err.cause(), "Simulated failure");
  assert_eq!(err.suggestion().unwrap(), "Do not pass fail=true");
  assert_eq!(err.metadata().get("context").unwrap(), "integration_test");
}
