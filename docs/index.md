# Cirious Codex Documentation

Welcome to the official documentation for **Cirious Codex**, the foundational diagnostic, logging, and configuration framework for the Cirious ecosystem.

## Overview

The `cirious_codex` workspace provides a robust set of modular crates designed to handle the core operational requirements of any high-performance application. It guarantees consistent error tracing, rich terminal interactions, structured configuration parsing, and highly customizable logging.

## Core Modules

The ecosystem is divided into the following specialized modules:

- **[Architecture & Design Philosophy](architecture.md)**
  An overview of how the internal components fit together seamlessly under the `cirious_codex` facade crate.

- **[Configuration Management](configuration.md)**
  Learn how to use `ConfigBuilder` to merge JSON, TOML, YAML, RON files with environment variables into strongly-typed structures.

- **[Error & Diagnostic Handling](error_handling.md)**
  Understand how `CodexError` and `CodexOk` work together to provide backtraces, actionable suggestions, and rich metadata.

- **[Logging & Observability](logging.md)**
  Dive into custom `Dispatchers` and `Formatters` to shape your logs for the terminal or structured external observability platforms.

## Getting Started

To get a hands-on feel for how these modules work, we highly recommend checking out the `examples/` directory in the root of the project:

- `01_quickstart.rs`
- `02_custom_logging.rs`
- `03_loading_config.rs`
- `04_full_ecosystem.rs`
