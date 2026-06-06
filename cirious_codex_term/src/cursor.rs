use std::io::{stdout, Write};

/// A utility struct for manipulating the terminal cursor.
///
/// Provides methods to move the cursor, change its visibility, and
/// save/restore its position using ANSI escape sequences.
///
/// # Examples
///
/// ```rust,no_run
/// use cirious_codex_term::cursor::Cursor;
///
/// Cursor::set_position(10, 5);
/// Cursor::hide();
/// ```
pub struct Cursor;

impl Cursor {
  /// Moves the cursor up by the specified number of lines.
  pub fn up(n: u16) {
    print!("\x1b[{}A", n);
  }

  /// Moves the cursor down by the specified number of lines.
  pub fn down(n: u16) {
    print!("\x1b[{}B", n);
  }

  /// Moves the cursor right by the specified number of columns.
  pub fn right(n: u16) {
    print!("\x1b[{}C", n);
  }

  /// Moves the cursor left by the specified number of columns.
  pub fn left(n: u16) {
    print!("\x1b[{}D", n);
  }

  /// Sets the absolute position of the cursor.
  ///
  /// `x` represents the column (1-based) and `y` represents the row (1-based).
  pub fn set_position(x: u16, y: u16) {
    print!("\x1b[{};{}H", y, x);
  }

  /// Hides the terminal cursor.
  pub fn hide() {
    print!("\x1b[?25l");
  }

  /// Shows the terminal cursor.
  pub fn show() {
    print!("\x1b[?25h");
  }

  /// Saves the current cursor position.
  pub fn save_position() {
    print!("\x1b[s");
  }

  /// Restores the cursor to the last saved position.
  pub fn restore_position() {
    print!("\x1b[u");
  }

  /// Flushes the standard output buffer.
  ///
  /// Terminal escape sequences are buffered by default. Calling this
  /// ensures that all cursor movements are executed immediately.
  pub fn flush() {
    let _ = stdout().flush();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cursor_movement_not_panicking() {
    // If your cursor functions write directly to stdout:
    Cursor::set_position(10, 10);
    Cursor::up(2);
    Cursor::hide();
    Cursor::show();
  }
}
