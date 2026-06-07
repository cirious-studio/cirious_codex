# Cli Integration

The `cirious_codex_cli` crate is designed to be the entry point for your application. It provides a powerful CLI router and configuration loader, enabling you to build production-ready microservices and CLI tools with ease.

---

## Features
Choose the configuration format that best fits your needs:
- `config_ron` - RON configuration support
- `config_json` - JSON configuration support
- `config_toml` - TOML configuration support
- `config_yaml` - YAML configuration support
- `config_full` - Full configuration support (all formats)

---

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_cli = { version = "0.1.0", features = ["config_full"] }
serde = { version = "1.0", features = ["derive"] }
clap = { version = "4.5", features = ["derive"] }
```
## Basic Implementation

```rust
use cirious_codex_cli::{execute_cli_with_config, CodexCommand, GlobalArgs};
use clap::{Parser, Subcommand};
use serde::Deserialize;

// 1. Define your configuration structure
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub database_url: String,
}

// 2. Define your CLI arguments and subcommands
#[derive(Parser, Debug)]
pub struct AppCLI {
    // GlobalArgs provides standard flags out-of-the-box (e.g., logging, config path)
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Subcommand, Debug)]
pub enum AppCommands {
    Start { port: u16 },
    Ping,
}

// 3. Implement CodexCommand to link the global args
impl CodexCommand for AppCLI {
    fn global_args(&self) -> &GlobalArgs {
        &self.global
    }
}

fn main() {
    // 4. Execute the router!
    execute_cli_with_config::<AppCLI, AppConfig, _>(|cli, config_opt| {
        if let Some(config) = config_opt {
            println!("Configuration loaded! Connecting to database at: {}", config.database_url);
        } else {
            println!("Warning: No configuration file provided. Using defaults.");
        }

        match cli.command {
            AppCommands::Start { port } => println!("Starting server on port {}", port),
            AppCommands::Ping => println!("Pong!"),
        }
    });
}
```

## Usage Examples
### *Running without a config file*:

```Bash
$ cargo run -- start 8080

[Info] Bootstrapping Logger in Info mode
Warning: No configuration file provided. Using defaults.
Starting server on port 8080
```

### Running with a TOML config file

> **Note:** The `GlobalArgs` struct automatically provides the `--config` flag.

```Bash
$ cargo run -- --config app.toml start 8080

[Info] Bootstrapping Logger in Info mode
Configuration loaded! Connecting to database at: postgres://localhost:5432/mydb
Starting server on port 8080
```

---

*Note: Since cirious_codex_cli is generic over the config type, you can use it with any configuration struct that implements serde::Deserialize.*
