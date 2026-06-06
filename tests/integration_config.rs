use cirious_codex::codex_config::{ConfigBuilder, Deserialize};
use cirious_codex_config::format::ConfigFormat;

mod common;

#[derive(Debug, Deserialize, PartialEq)]
struct TestConfig {
  host: String,
  port: u16,
  is_active: bool,
}

#[test]
fn test_config_builder_json_and_env() {
  common::setup();

  let base_json = r#"{
    "host": "localhost",
    "port": 8080,
    "is_active": false
  }"#;

  common::with_env_var("TEST_PORT", "9999", || {
    common::with_env_var("TEST_IS_ACTIVE", "true", || {
      let result = ConfigBuilder::new()
        .add_source(base_json, ConfigFormat::Json)
        .expect("Failed to parse JSON")
        .value
        .add_env_prefix("TEST_")
        .build::<TestConfig>()
        .expect("Failed to build config")
        .value;

      assert_eq!(result.host, "localhost");
      assert_eq!(result.port, 9999);
      assert!(result.is_active);
    });
  });
}

#[test]
fn test_config_builder_invalid_json() {
  common::setup();

  let invalid_json = r#"{ "host": "localhost", }"#; // Trailing comma is invalid JSON

  let result = ConfigBuilder::new().add_source(invalid_json, ConfigFormat::Json);
  assert!(result.is_err(), "Should have failed to parse invalid JSON");

  let err = result.unwrap_err();
  assert!(err.metadata().contains_key("format"));
}
