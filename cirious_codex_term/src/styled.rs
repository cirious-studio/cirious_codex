use std::fmt;

use crate::{Color, Style};

/// A deferred evaluation builder for ANSI terminal styles.
///
/// Instead of allocating strings immediately, `StyledText` stores the
/// requested styles and injects the ANSI escape codes directly into the
/// buffer when `std::fmt::Display` is invoked. This ensures maximum
/// performance and prevents duplicate reset codes.
pub struct StyledText<T> {
  /// The underlying text or data to be styled.
  pub text: T,

  /// The foreground color to be applied, if any.
  pub fg: Option<Color>,

  /// The background color to be applied, if any.
  pub bg: Option<Color>,

  /// A list of text formatting styles to be applied.
  pub styles: Vec<Style>,
}

impl<T> StyledText<T> {
  /// Creates a new `StyledText` instance wrapping the provided text.
  pub fn new(text: T) -> Self {
    Self {
      text,
      fg: None,
      bg: None,
      styles: Vec::new(),
    }
  }

  /// Sets the foreground color to the specified `Color`.
  pub fn color(mut self, color: Color) -> Self {
    self.fg = Some(color);
    self
  }

  /// Sets the background color to the specified `Color`.
  pub fn bg(mut self, color: Color) -> Self {
    self.bg = Some(color);
    self
  }

  /// Sets the foreground color using an 8-bit (256-color) palette index.
  pub fn fixed(mut self, n: u8) -> Self {
    self.fg = Some(Color::Fixed(n));
    self
  }

  /// Sets the foreground color using an RGB TrueColor value.
  pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    self.fg = Some(Color::Rgb(r, g, b));
    self
  }

  /// Sets the background color using an 8-bit (256-color) palette index.
  pub fn bg_fixed(mut self, n: u8) -> Self {
    self.bg = Some(Color::Fixed(n));
    self
  }

  /// Sets the background color using an RGB TrueColor value.
  pub fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
    self.bg = Some(Color::Rgb(r, g, b));
    self
  }

  /// Clears all applied styles, foreground, and background colors.
  pub fn reset(mut self) -> Self {
    self.fg = Some(Color::Reset);
    self.bg = Some(Color::Reset);
    self.styles.clear();
    self
  }

  /// Sets the foreground color to black.
  pub fn black(mut self) -> Self {
    self.fg = Some(Color::Black);
    self
  }

  /// Sets the foreground color to bright black (gray).
  pub fn bright_black(mut self) -> Self {
    self.fg = Some(Color::BrightBlack);
    self
  }

  /// Sets the foreground color to bright red.
  pub fn bright_red(mut self) -> Self {
    self.fg = Some(Color::BrightRed);
    self
  }

  /// Sets the foreground color to bright green.
  pub fn bright_green(mut self) -> Self {
    self.fg = Some(Color::BrightGreen);
    self
  }

  /// Sets the foreground color to bright yellow.
  pub fn bright_yellow(mut self) -> Self {
    self.fg = Some(Color::BrightYellow);
    self
  }

  /// Sets the foreground color to bright blue.
  pub fn bright_blue(mut self) -> Self {
    self.fg = Some(Color::BrightBlue);
    self
  }

  /// Sets the foreground color to bright magenta.
  pub fn bright_magenta(mut self) -> Self {
    self.fg = Some(Color::BrightMagenta);
    self
  }

  /// Sets the foreground color to bright cyan.
  pub fn bright_cyan(mut self) -> Self {
    self.fg = Some(Color::BrightCyan);
    self
  }

  /// Sets the foreground color to bright white.
  pub fn bright_white(mut self) -> Self {
    self.fg = Some(Color::BrightWhite);
    self
  }

  /// Sets the foreground color to red.
  pub fn red(mut self) -> Self {
    self.fg = Some(Color::Red);
    self
  }

  /// Sets the foreground color to green.
  pub fn green(mut self) -> Self {
    self.fg = Some(Color::Green);
    self
  }

  /// Sets the foreground color to yellow.
  pub fn yellow(mut self) -> Self {
    self.fg = Some(Color::Yellow);
    self
  }

  /// Sets the foreground color to blue.
  pub fn blue(mut self) -> Self {
    self.fg = Some(Color::Blue);
    self
  }

  /// Sets the foreground color to magenta.
  pub fn magenta(mut self) -> Self {
    self.fg = Some(Color::Magenta);
    self
  }

  /// Sets the foreground color to cyan.
  pub fn cyan(mut self) -> Self {
    self.fg = Some(Color::Cyan);
    self
  }

  /// Sets the foreground color to white.
  pub fn white(mut self) -> Self {
    self.fg = Some(Color::White);
    self
  }

  /// Sets the background color to black.
  pub fn bg_black(mut self) -> Self {
    self.bg = Some(Color::Black);
    self
  }

  /// Sets the background color to bright black (gray).
  pub fn bg_bright_black(mut self) -> Self {
    self.bg = Some(Color::BrightBlack);
    self
  }

  /// Sets the background color to bright red.
  pub fn bg_bright_red(mut self) -> Self {
    self.bg = Some(Color::BrightRed);
    self
  }

  /// Sets the background color to bright green.
  pub fn bg_bright_green(mut self) -> Self {
    self.bg = Some(Color::BrightGreen);
    self
  }

  /// Sets the background color to bright yellow.
  pub fn bg_bright_yellow(mut self) -> Self {
    self.bg = Some(Color::BrightYellow);
    self
  }

  /// Sets the background color to bright blue.
  pub fn bg_bright_blue(mut self) -> Self {
    self.bg = Some(Color::BrightBlue);
    self
  }

  /// Sets the background color to bright magenta.
  pub fn bg_bright_magenta(mut self) -> Self {
    self.bg = Some(Color::BrightMagenta);
    self
  }

  /// Sets the background color to bright cyan.
  pub fn bg_bright_cyan(mut self) -> Self {
    self.bg = Some(Color::BrightCyan);
    self
  }

  /// Sets the background color to bright white.
  pub fn bg_bright_white(mut self) -> Self {
    self.bg = Some(Color::BrightWhite);
    self
  }

  /// Sets the background color to red.
  pub fn bg_red(mut self) -> Self {
    self.bg = Some(Color::Red);
    self
  }

  /// Sets the background color to green.
  pub fn bg_green(mut self) -> Self {
    self.bg = Some(Color::Green);
    self
  }

  /// Sets the background color to yellow.
  pub fn bg_yellow(mut self) -> Self {
    self.bg = Some(Color::Yellow);
    self
  }

  /// Sets the background color to blue.
  pub fn bg_blue(mut self) -> Self {
    self.bg = Some(Color::Blue);
    self
  }

  /// Sets the background color to magenta.
  pub fn bg_magenta(mut self) -> Self {
    self.bg = Some(Color::Magenta);
    self
  }

  /// Sets the background color to cyan.
  pub fn bg_cyan(mut self) -> Self {
    self.bg = Some(Color::Cyan);
    self
  }

  /// Sets the background color to white.
  pub fn bg_white(mut self) -> Self {
    self.bg = Some(Color::White);
    self
  }

  /// Adds the underline style to the text.
  pub fn underline(mut self) -> Self {
    self.styles.push(Style::Underline);
    self
  }

  /// Decreases the intensity of the text color.
  pub fn dim(mut self) -> Self {
    self.styles.push(Style::Dim);
    self
  }

  /// Adds the italic style to the text.
  pub fn italic(mut self) -> Self {
    self.styles.push(Style::Italic);
    self
  }

  /// Adds the blink style to the text.
  pub fn blink(mut self) -> Self {
    self.styles.push(Style::Blink);
    self
  }

  /// Reverses the foreground and background colors.
  pub fn reverse(mut self) -> Self {
    self.styles.push(Style::Reverse);
    self
  }

  /// Hides the text completely.
  pub fn hidden(mut self) -> Self {
    self.styles.push(Style::Hidden);
    self
  }

  /// Adds a strikethrough line to the text.
  pub fn strikethrough(mut self) -> Self {
    self.styles.push(Style::Strikethrough);
    self
  }

  /// Adds the bold style to the text.
  pub fn bold(mut self) -> Self {
    self.styles.push(Style::Bold);
    self
  }
}

// Display implementation ensuring a single Reset at the end
impl<T: fmt::Display> fmt::Display for StyledText<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for style in &self.styles {
      write!(f, "{}", style.to_str())?;
    }
    if let Some(fg) = &self.fg {
      write!(f, "{}", fg.to_fg_str())?;
    }
    if let Some(bg) = &self.bg {
      write!(f, "{}", bg.to_bg_str())?;
    }
    write!(f, "{}", self.text)?;
    write!(f, "{}", Style::Reset.to_str())
  }
}
