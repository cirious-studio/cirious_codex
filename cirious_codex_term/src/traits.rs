use crate::color::Color;
use crate::style::Style;

/// Allows formatting strings effortlessly
pub trait StyleExt {
  fn color(self, color: Color) -> String;
  fn bg(self, color: Color) -> String;

  // Quick colors
  fn black(self) -> String;
  fn red(self) -> String;
  fn green(self) -> String;
  fn yellow(self) -> String;
  fn blue(self) -> String;
  fn magenta(self) -> String;
  fn cyan(self) -> String;
  fn white(self) -> String;
  fn rgb(self, r: u8, g: u8, b: u8) -> String;

  // Quick styles
  fn bold(self) -> String;
  fn italic(self) -> String;
  fn underline(self) -> String;
  fn blink(self) -> String;
  fn strikethrough(self) -> String;
  fn dim(self) -> String;

  fn reset(self) -> String;
}

impl<T: AsRef<str>> StyleExt for T {
  fn color(self, color: Color) -> String {
    format!("{}{}{}", color.to_fg_str(), self.as_ref(), "\x1b[0m")
  }

  fn bg(self, color: Color) -> String {
    format!("{}{}{}", color.to_bg_str(), self.as_ref(), "\x1b[0m")
  }

  fn black(self) -> String {
    self.color(Color::Black)
  }
  fn red(self) -> String {
    self.color(Color::Red)
  }
  fn green(self) -> String {
    self.color(Color::Green)
  }
  fn yellow(self) -> String {
    self.color(Color::Yellow)
  }
  fn blue(self) -> String {
    self.color(Color::Blue)
  }
  fn magenta(self) -> String {
    self.color(Color::Magenta)
  }
  fn cyan(self) -> String {
    self.color(Color::Cyan)
  }
  fn white(self) -> String {
    self.color(Color::White)
  }
  fn rgb(self, r: u8, g: u8, b: u8) -> String {
    self.color(Color::Rgb(r, g, b))
  }

  fn bold(self) -> String {
    format!("{}{}{}", Style::Bold.to_str(), self.as_ref(), "\x1b[0m")
  }
  fn italic(self) -> String {
    format!("{}{}{}", Style::Italic.to_str(), self.as_ref(), "\x1b[0m")
  }
  fn underline(self) -> String {
    format!("{}{}{}", Style::Underline.to_str(), self.as_ref(), "\x1b[0m")
  }
  fn blink(self) -> String {
    format!("{}{}{}", Style::Blink.to_str(), self.as_ref(), "\x1b[0m")
  }
  fn strikethrough(self) -> String {
    format!("{}{}{}", Style::Strikethrough.to_str(), self.as_ref(), "\x1b[0m")
  }
  fn dim(self) -> String {
    format!("{}{}{}", Style::Dim.to_str(), self.as_ref(), "\x1b[0m")
  }

  fn reset(self) -> String {
    format!("{}{}", self.as_ref(), Style::Reset.to_str())
  }
}
