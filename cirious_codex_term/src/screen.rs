/// A utility struct for manipulating the terminal screen.
///
/// Provides methods to clear the terminal window, the current line,
/// or specific portions of the screen using ANSI escape sequences.
///
/// # Examples
///
/// ```rust,no_run
/// use cirious_codex_term::screen::Screen;
///
/// Screen::clear();
/// ```
pub struct Screen;

impl Screen {
  /// Clears the entire screen and moves the cursor to the home position (top-left).
  ///
  /// This method writes the ANSI escape sequence `\x1b[2J\x1b[H` directly to standard output.
  pub fn clear() {
    print!("\x1b[2J\x1b[H");
  }

  /// Clears the screen from the current cursor position to the end of the display.
  ///
  /// This method writes the ANSI escape sequence `\x1b[J` directly to standard output.
  pub fn clear_to_end() {
    print!("\x1b[J");
  }

  /// Clears the entire current line without moving the cursor.
  ///
  /// This method writes the ANSI escape sequence `\x1b[2K` directly to standard output.
  pub fn clear_line() {
    print!("\x1b[2K");
  }
}
