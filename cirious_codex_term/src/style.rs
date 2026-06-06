/// Represents ANSI text formatting styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
  /// Makes the text bold or increased intensity.
  Bold,

  /// Decreases the intensity of the text.
  Dim,

  /// Makes the text italic (not supported in all terminals).
  Italic,

  /// Underlines the text (not supported in all terminals).
  Underline,

  /// Makes the text blink (not supported in all terminals).
  Blink,

  /// Reverses the foreground and background colors.
  Reverse,

  /// Hides the text (not supported in all terminals).
  Hidden,

  /// Puts a line through the text (not supported in all terminals).
  Strikethrough,

  /// Resets all styles to default.
  Reset,
}

impl Style {
  /// Returns the ANSI escape sequence for the text style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use cirious_codex_term::Style;
  ///
  /// assert_eq!(Style::Bold.to_str(), "\x1b[1m");
  /// ```
  pub fn to_str(&self) -> &'static str {
    match self {
      Style::Bold => "\x1b[1m",
      Style::Dim => "\x1b[2m",
      Style::Italic => "\x1b[3m",
      Style::Underline => "\x1b[4m",
      Style::Blink => "\x1b[5m",
      Style::Reverse => "\x1b[7m",
      Style::Hidden => "\x1b[8m",
      Style::Strikethrough => "\x1b[9m",
      Style::Reset => "\x1b[0m",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_style_codes() {
    assert_eq!(Style::Bold.to_str(), "\x1b[1m");
    assert_eq!(Style::Reset.to_str(), "\x1b[0m");
    assert_eq!(Style::Dim.to_str(), "\x1b[2m");
  }
}
