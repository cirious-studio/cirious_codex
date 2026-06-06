# Logging & Observability

The `cirious_codex_logger` crate is designed to be the ultimate observability bedrock. It decouples the act of generating a log from the act of formatting and dispatching it.

## Macros

The library provides the standard suite of logging macros:
- `trace!`
- `debug!`
- `info!`
- `warn!`
- `error!`

Currently, these macros route to a basic internal dispatcher, but the architecture supports deep customization for advanced applications.

## Formatters

Formatters dictate **how** a log `Record` is serialized.
- `JsonFormatter`: Perfect for centralized logging systems like Datadog, ELK, or AWS CloudWatch. It outputs structured JSON.
- `HumanReadableFormatter`: A clean, readable string format for raw text files or basic terminals.
- `StyledTerminalFormatter`: Leverages `cirious_codex_term` under the hood to inject ANSI color codes and bold styling, making your local development terminal pop with color-coded severities.

## Dispatchers

Dispatchers dictate **where** the serialized string goes.
- `StdoutDispatcher`: Writes directly to standard output.
- `StderrDispatcher`: Writes directly to standard error (crucial for error-level logs in CLI applications).

## Example Usage

You can wire these together to create highly customized observability pipelines:

```rust
use cirious_codex::codex_logger::{
    Dispatcher, Level, Record, StdoutDispatcher, StyledTerminalFormatter
};

let dispatcher = StdoutDispatcher::new(StyledTerminalFormatter);

let record = Record {
    level: Level::Warn,
    args: format_args!("High memory usage detected!"),
};

// Dispatches a boldly colored warning to the terminal
dispatcher.dispatch(&record);
```
