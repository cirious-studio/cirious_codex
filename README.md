<div align="center">

# ⚙️ Cirious Codex

**Next-Generation Rust Tooling & Ecosystem Bedrock**

[![Status](https://img.shields.io/badge/Status-v0.1.0_Released-success.svg)]() [![CI](https://github.com/cirious-studio/cirious_codex/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex.svg)](https://crates.io/crates/cirious_codex) [![Docs.rs](https://docs.rs/cirious_codex/badge.svg)](https://docs.rs/cirious_codex) [![Language](https://img.shields.io/badge/Language-Rust-black?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

<br />

> 🚀 **Note:** The foundational architecture (v0.1.0) is officially stable and published. We have successfully implemented our highly-modular facade pattern, CI/CD pipelines, and deep test integrations.

</div>

---

## 📖 Overview

**Cirious Codex** is an ambitious, modern workspace dedicated to building high-performance Rust libraries. The primary architecture is designed to serve as the central hub and foundational bedrock for our ecosystem tools, prioritizing modularity, safety, and uncompromising execution speed.

Through the elegant use of the **Facade Pattern**, consumers only need to import the single `cirious_codex` crate. Under the hood, feature flags seamlessly orchestrate specialized, hyper-optimized micro-crates.

---

## 🧱 Ecosystem Crates

The workspace is composed of the following core libraries:

- **`cirious_codex_config`**: A robust configuration engine that seamlessly merges JSON/TOML/YAML/RON files with environment variables into strongly-typed structs.
- **`cirious_codex_logger`**: The ultimate observability bedrock, providing customizable `Formatters` (JSON, Human, Styled) and `Dispatchers`.
- **`cirious_codex_result`**: A revolutionary approach to error handling. `CodexError` captures system backtraces, exact code locations, and actionable diagnostic suggestions natively.
- **`cirious_codex_term`**: A lightweight, dependency-free terminal styling utility for rich ANSI text outputs.
- **`cirious_codex_cli`**: Premier entrypoint library for building production-ready CLIs and microservices in the Cirious ecosystem, with automated argument parsing, logger bootstrapping, and configuration loading.
- **`cirious_codex_metrics`**: A simplified application telemetry facade and native Prometheus exporter, offering decoupled metric recording (counters, gauges, histograms) with automated background listener scaffolding.

---

## 📚 Documentation & Examples

We believe in documentation-driven development. To learn how to use the Cirious Codex ecosystem, please explore:

- **[The Official Architecture (`/docs/architecture.md`)](./docs/architecture.md)**: A deep dive into how the internal components fit together seamlessly under the `cirious_codex` facade crate.
- **[The Official Docs (`/docs/index.md`)](./docs/index.md)**: Deep dives into the architecture, configuration resolution, error handling, and logging dispatchers.
- **[The Examples (`/examples`)](./examples)**: Ready-to-run code snippets showcasing how the modules operate independently and as a cohesive ecosystem (`cargo run --example 05_full_ecosystem --all-features`).

---

## 🚧 Roadmap

Our workspace follows a structured evolutionary path. With the foundational layer stabilized, we are shifting focus from core primitives to ecosystem expansion:

**Phase 1: Foundation & Observability (v0.1.0 - Current Release)**
- [x] Repository initialization and workspace declaration.
- [x] Initial release of core data-oriented libraries (`logger`, `result`, `config`, `term`).
- [x] Architect and integrate `cirious_codex_cli` for rapid application scaffolding.
- [x] Develop `cirious_codex_metrics` for unified telemetry and Prometheus exposition.
- [x] Establish strict CI/CD pipelines and professional workspace integration tests.
- [x] Define architectural contribution guidelines and comprehensive documentation.
- [x] Publish the unified `cirious_codex` facade to `crates.io` as a workspace.

**Phase 2: Ecosystem Expansion (Upcoming)**
- [ ] **`cirious_codex_net`**: Frictionless HTTP/microservice integrations (standardized middlewares, trace injection, and automatic RFC 7807 error responses).
- [ ] **`cirious_codex_fs`**: Resilient, async-first file system I/O wrappers returning rich, actionable `CodexError` diagnostics.
- [ ] **`cirious_codex_tasks`**: In-memory background job orchestrator with built-in Prometheus telemetry and automated logging.

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by Cirious Studio</sub>
</div>
