# Error & Diagnostic Handling

The `cirious_codex_result` crate fundamentally upgrades Rust's native error handling by providing rich, context-aware diagnostic payloads.

## CodexError

Instead of simple strings or enums, failures in the Cirious ecosystem return a `CodexError`. This error acts as a detailed diagnostic document.

When you use `CodexError::builder()`, it automatically injects `#[track_caller]` and system capabilities to capture:
1. The exact **file and line number** where the error was generated.
2. A full system **Backtrace**.

### Adding Context

You can chain builder methods to provide hints and structured metadata:

```rust
use cirious_codex_result::CodexError;

fn connect_db() -> Result<(), CodexError> {
    Err(
        CodexError::builder("DB_TIMEOUT", "Connection to database timed out")
            .with_suggestion("Verify if the PostgreSQL container is running on port 5432.")
            .with_meta("timeout_ms", "5000")
            // .build() is not needed, builder returns Self
    )
}
```

## CodexOk

Success states aren't left behind. Wrapping your success values in a `CodexOk` captures the location where the operation successfully finished and allows attaching metadata to successful executions (e.g., execution durations, payload sizes).

The `codex_ok!` macro provides an ergonomic way to return successes:

```rust
use cirious_codex_result::{codex_ok, Result};

fn compute_data() -> Result<u32> {
    // ... complex logic ...
    codex_ok!(42) // Wraps 42 in CodexOk and returns it
}
```

*Note: Since `CodexOk` encapsulates the value, returning `Result<T>` from your `main()` function requires wrapping your logic in an inner `run()` function, as custom types currently do not implement the `Termination` trait required by the Rust compiler for `main()`.*
