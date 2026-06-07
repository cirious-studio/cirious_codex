//! # Cli Management with Cirious Codex
//!
//! This example demonstrates how to build a custom CLI application
//! using `cirious_codex_cli`.
//!
//! You can run this example with:
//! `cargo run --example 04_custom_cli --all-features`

use cirious_codex::codex_cli::*;
use clap::{Parser, Subcommand};

#[cfg(feature = "cli_config")]
use serde::Deserialize;

// 1. Strongly-typed configuration
#[cfg(feature = "cli_config")]
#[derive(Deserialize, Debug)]
struct MicroserviceConfig {
  pub server_port: u16,
  pub database_url: String,
  pub worker_threads: usize,
}

// 2. CLI structure with complete metadata
#[derive(Parser, Debug)]
#[command(
  name = "auth_service",
  version = "1.0",
  about = "Cirious Authentication Microservice"
)]
struct AuthCLI {
  #[command(flatten)]
  global: GlobalArgs,

  #[command(subcommand)]
  command: AuthCommands,
}

// 3. Subcommands with their specific arguments
#[derive(Subcommand, Debug)]
enum AuthCommands {
  /// Starts the HTTP server
  Serve {
    /// Override the default configuration server port
    #[arg(short, long)]
    port: Option<u16>,
  },
  /// Runs database migrations
  Migrate {
    /// Drop all tables before migrating (DANGEROUS)
    #[arg(long)]
    reset: bool,
  },
  /// Starts a background background worker
  Worker,
}

// 4. Implement the Engine Contract
impl CodexCommand for AuthCLI {
  fn global_args(&self) -> &GlobalArgs {
    &self.global
  }
}

fn main() {
  #[cfg(not(feature = "cli_config"))]
  execute_cli::<AuthCLI, _>(|cli| {
    // Sub-command Router
    match cli.command {
      AuthCommands::Serve { port } => {
        if let Some(port) = port {
          println!("Action: Starting HTTP Server on port {} (CLI argument)!", port);
        } else {
          println!("Action: Starting HTTP Server on default port! (No configuration file provided)");
        }
      }
      AuthCommands::Migrate { reset } => {
        if reset {
          println!("WARNING: Dropping all tables before migration!");
        }
        println!("Action: Executing SQL migrations...");
      }
      AuthCommands::Worker => {
        println!("Action: Starting background job processor...");
      }
    }
  });

  #[cfg(feature = "cli_config")]
  execute_cli_with_config::<AuthCLI, MicroserviceConfig, _>(|cli, config_opt| {
    // Enforce configuration requirement
    let config = config_opt.expect("CRITICAL: A valid configuration file is required!");

    println!("\n--- Application Logic Starts Here ---");
    println!("Connected to Database: {}", config.database_url);

    // Sub-command Router
    match cli.command {
      AuthCommands::Serve { port } => {
        // Feature: CLI argument overrides configuration file!
        let active_port = port.unwrap_or(config.server_port);
        println!(
          "Action: Starting HTTP Server on port {} with {} worker threads",
          active_port, config.worker_threads
        );
      }
      AuthCommands::Migrate { reset } => {
        if reset {
          println!("WARNING: Dropping all tables before migration!");
        }
        println!("Action: Executing SQL migrations...");
      }
      AuthCommands::Worker => {
        println!("Action: Starting background job processor...");
      }
    }
  });
}
