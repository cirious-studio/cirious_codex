pub struct Screen;

impl Screen {
  /// Clears the entire screen and moves cursor to home
  pub fn clear() {
    print!("\x1b[2J\x1b[H");
  }

  /// Clears from cursor to the end of the screen
  pub fn clear_to_end() {
    print!("\x1b[J");
  }

  /// Clears the current line
  pub fn clear_line() {
    print!("\x1b[2K");
  }
}
