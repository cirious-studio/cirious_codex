# Configuration Management

The `cirious_codex_config` crate provides a highly ergonomic and powerful configuration resolution engine based on `serde`.

## The ConfigBuilder

The core of the configuration system is the `ConfigBuilder`. It allows you to progressively layer configuration sources. Later sources recursively override earlier ones.

```rust
use cirious_codex::codex_config::{ConfigBuilder, Deserialize};
use cirious_codex_config::format::ConfigFormat;

#[derive(Deserialize)]
struct AppConfig {
    port: u16,
}

let json_content = r#"{ "port": 8080 }"#;

let config = ConfigBuilder::new()
    .add_source(json_content, ConfigFormat::Json).unwrap().value
    .add_env_prefix("APP_") // Overrides port if APP_PORT is set
    .build::<AppConfig>().unwrap().value;

assert_eq!(config.port, 8080);
```

## Supported Formats

Currently, `ConfigBuilder` parses raw strings based on the `ConfigFormat` enum:
- `ConfigFormat::Json`
- `ConfigFormat::Toml`
- `ConfigFormat::Yaml`
- `ConfigFormat::Ron`

*Note: The actual file reading must be handled by the application (e.g., using `std::fs::read_to_string`), making the builder completely uncoupled from the filesystem.*

## Environment Variables

The `.add_env_prefix("PREFIX_")` method scans the OS environment variables. If it finds a match, it strips the prefix, converts the key to lowercase, and injects it into the configuration tree. This makes configuring production deployments via Docker or Kubernetes incredibly straightforward.
