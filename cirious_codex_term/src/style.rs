/// ANSI Text Styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
  Bold,
  Dim,
  Italic,
  Underline,
  Blink,
  Reverse,
  Hidden,
  Strikethrough,
  Reset,
}

impl Style {
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
