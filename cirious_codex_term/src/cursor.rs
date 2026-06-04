use std::io::{stdout, Write};

pub struct Cursor;

impl Cursor {
  pub fn up(n: u16) {
    print!("\x1b[{}A", n);
  }
  pub fn down(n: u16) {
    print!("\x1b[{}B", n);
  }
  pub fn right(n: u16) {
    print!("\x1b[{}C", n);
  }
  pub fn left(n: u16) {
    print!("\x1b[{}D", n);
  }

  pub fn set_position(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x);
  }

  pub fn hide() {
    print!("\x1b[?25l");
  }
  pub fn show() {
    print!("\x1b[?25h");
  }

  pub fn save_position() {
    print!("\x1b[s");
  }
  pub fn restore_position() {
    print!("\x1b[u");
  }

  pub fn flush() {
    let _ = stdout().flush();
  }
}
