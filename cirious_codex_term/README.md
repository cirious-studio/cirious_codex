<div align="center">

# 🖥️ Cirious Codex Term

**Native ANSI Terminal Control & Formatting**

[![CI](https://github.com/cirious-studio/cirious_codex_term/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex_term/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex_term.svg)](https://crates.io/crates/cirious_codex_term) [![Docs.rs](https://docs.rs/cirious_codex_term/badge.svg)](https://docs.rs/cirious_codex_term) [![Language](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Cirious Codex Term** is a highly optimized, dependency-free foundational library for terminal manipulation. It provides native ANSI escape sequence generation for colors, text styling, and full cursor control. 

Designed to be the ultimate bedrock for CLI tools within the Cirious ecosystem, prioritizing zero-cost abstractions and flawless terminal integration.

## ✨ Features

- Core color manipulation (16 colors, 256 colors, TrueColor RGB).
- Text formatting traits (Bold, Italic, Blink, Dim, etc.).
- Absolute and relative cursor positioning.
- Screen manipulation and clearing.

---

## 🚀 Quick Start
 
Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_term = "0.1"
```

And then in your code:

```rust
use cirious_codex_term::{Color, Cursor, Screen, Style, StyleExt};
 
fn main() {
  // Clear the screen
  Screen::clear();
   
  // Move cursor and print styled text
  Cursor::set_position(5, 5);
  print!(
    "{} {}Hello, Codex Term!{}",
    Style::Bold.to_str(),
    Color::Cyan.to_fg_str(),
    Style::Reset.to_str()
  );
   
  // Use extension traits for fluent styling
  println!("{}", "This text is red and underlined".red().underline());
   
  // Hide cursor
  Cursor::hide();
   
  // Move cursor and print again
  Cursor::set_position(10, 10);
  println!("{}", "Cursor is hidden!".green());
   
  // Show cursor
  Cursor::show();
}
```
---

## 🚧 Current Status & Roadmap

The architecture is currently being mapped out for the upcoming `v0.2` release:

- [ ] **Windows ANSI:** Auto-enable native ANSI support on Windows.
- [ ] **Terminal Dimensions:** Methods to query terminal width and height dynamically.
- [ ] **Raw Mode Toggle:** Ability to switch the terminal to raw mode for unbuffered, character-by-character input reading.
- [ ] **NO_COLOR Standard:** Automatic respect for the `NO_COLOR` environment variable to disable styling dynamically.
- [ ] **Async Key Events:** Foundational support for reading non-blocking keyboard and mouse events.


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
