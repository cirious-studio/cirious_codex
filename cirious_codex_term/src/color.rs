/// Represents ANSI terminal foreground and background colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  /// Standard ANSI black (`\x1b[30m`).
  Black,

  /// Standard ANSI red (`\x1b[31m`).
  Red,

  /// Standard ANSI green (`\x1b[32m`).
  Green,

  /// Standard ANSI yellow (`\x1b[33m`).
  Yellow,

  /// Standard ANSI blue (`\x1b[34m`).
  Blue,

  /// Standard ANSI magenta (`\x1b[35m`).
  Magenta,

  /// Standard ANSI cyan (`\x1b[36m`).
  Cyan,

  /// Standard ANSI white (`\x1b[37m`).
  White,

  /// Standard ANSI bright black (`\x1b[90m`).
  BrightBlack,

  /// Standard ANSI bright red (`\x1b[91m`).
  BrightRed,

  /// Standard ANSI bright green (`\x1b[92m`).
  BrightGreen,

  /// Standard ANSI bright yellow (`\x1b[93m`).
  BrightYellow,

  /// Standard ANSI bright blue (`\x1b[94m`).
  BrightBlue,

  /// Standard ANSI bright magenta (`\x1b[95m`).
  BrightMagenta,

  /// Standard ANSI bright cyan (`\x1b[96m`).
  BrightCyan,

  /// Standard ANSI bright white (`\x1b[97m`).
  BrightWhite,

  /// TrueColor RGB representation.
  Rgb(u8, u8, u8),

  /// 256-color palette (8-bit fixed colors).
  Fixed(u8),

  /// Reset all colors and styles to default.
  Reset,
}

impl Color {
  /// Returns the ANSI escape sequence for the foreground color.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use cirious_codex_term::Color;
  ///
  /// assert_eq!(Color::Red.to_fg_str(), "\x1b[31m");
  /// ```
  pub fn to_fg_str(&self) -> String {
    match self {
      Color::Black => "\x1b[30m".to_string(),
      Color::Red => "\x1b[31m".to_string(),
      Color::Green => "\x1b[32m".to_string(),
      Color::Yellow => "\x1b[33m".to_string(),
      Color::Blue => "\x1b[34m".to_string(),
      Color::Magenta => "\x1b[35m".to_string(),
      Color::Cyan => "\x1b[36m".to_string(),
      Color::White => "\x1b[37m".to_string(),
      Color::BrightBlack => "\x1b[90m".to_string(),
      Color::BrightRed => "\x1b[91m".to_string(),
      Color::BrightGreen => "\x1b[92m".to_string(),
      Color::BrightYellow => "\x1b[93m".to_string(),
      Color::BrightBlue => "\x1b[94m".to_string(),
      Color::BrightMagenta => "\x1b[95m".to_string(),
      Color::BrightCyan => "\x1b[96m".to_string(),
      Color::BrightWhite => "\x1b[97m".to_string(),
      Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
      Color::Fixed(n) => format!("\x1b[38;5;{}m", n),
      Color::Reset => "\x1b[39m".to_string(),
    }
  }

  /// Returns the ANSI escape sequence for the background color.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use cirious_codex_term::Color;
  ///
  /// assert_eq!(Color::Green.to_bg_str(), "\x1b[42m");
  /// ```
  pub fn to_bg_str(&self) -> String {
    match self {
      Color::Black => "\x1b[40m".to_string(),
      Color::Red => "\x1b[41m".to_string(),
      Color::Green => "\x1b[42m".to_string(),
      Color::Yellow => "\x1b[43m".to_string(),
      Color::Blue => "\x1b[44m".to_string(),
      Color::Magenta => "\x1b[45m".to_string(),
      Color::Cyan => "\x1b[46m".to_string(),
      Color::White => "\x1b[47m".to_string(),
      Color::BrightBlack => "\x1b[100m".to_string(),
      Color::BrightRed => "\x1b[101m".to_string(),
      Color::BrightGreen => "\x1b[102m".to_string(),
      Color::BrightYellow => "\x1b[103m".to_string(),
      Color::BrightBlue => "\x1b[104m".to_string(),
      Color::BrightMagenta => "\x1b[105m".to_string(),
      Color::BrightCyan => "\x1b[106m".to_string(),
      Color::BrightWhite => "\x1b[107m".to_string(),
      Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
      Color::Fixed(n) => format!("\x1b[48;5;{}m", n),
      Color::Reset => "\x1b[49m".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_standard_colors() {
    assert_eq!(Color::Red.to_fg_str(), "\x1b[31m");
    assert_eq!(Color::Green.to_bg_str(), "\x1b[42m");
  }

  #[test]
  fn test_rgb_colors() {
    // Adapt according to your RGB struct/methods
    let rgb_color = Color::Rgb(255, 100, 50);
    assert_eq!(rgb_color.to_fg_str(), "\x1b[38;2;255;100;50m");
  }
}
