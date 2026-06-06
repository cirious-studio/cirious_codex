//! # Cirious Codex
//!
//! `cirious_codex` is the foundational framework and facade crate for the entire
//! Cirious ecosystem. It aggregates highly optimized, specialized libraries into a
//! single, unified API surface, ensuring a consistent and robust developer experience.
//!
//! ## Overview
//!
//! By using `cirious_codex`, you gain immediate access to our ecosystem of tools
//! through optional feature flags. This allows you to pick and choose exactly what
//! you need without binary bloat.
//!
//! - **Configuration (`config` feature)**: A robust configuration management framework for
//!   loading, parsing, and validating application settings from multiple sources.
//! - **Logging (`logger` feature)**: Structured, high-performance logging built for
//!   observability and terminal output.
//! - **Result & Error Handling (`result` feature)**: A unified diagnostic and error
//!   tracking system designed to work flawlessly across all Cirious crates.
//! - **Terminal (`term` feature)**: Advanced terminal manipulation, styling, and
//!   interactive capabilities.
//!
//! ## Usage
//!
//! Enable only the features your application requires in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cirious_codex = { version = "0.1.0", features = ["config", "logger"] }
//! ```
//!
//! Or enable the full ecosystem:
//!
//! ```toml
//! [dependencies]
//! cirious_codex = { version = "0.1.0", features = ["full"] }
//! ```

#![warn(missing_docs)]

#[cfg(feature = "config")]
#[doc(inline)]
pub use cirious_codex_config as codex_config;

#[cfg(feature = "logger")]
#[doc(inline)]
pub use cirious_codex_logger as codex_logger;

#[cfg(feature = "result")]
#[doc(inline)]
pub use cirious_codex_result as codex_result;

#[cfg(feature = "term")]
#[doc(inline)]
pub use cirious_codex_term as codex_term;
