use cirious_codex_term::{Color, Cursor, Screen, Style};
// Assuming you have traits like Stylize or Colorize in `traits.rs` to allow `"text".red().bold()`
// use cirious_codex_term::traits::*;

fn main() {
  // Screen clearing
  Screen::clear();
  Cursor::set_position(1, 1);

  // Using raw styles and colors
  println!(
    "{} {} Hello, Codex Term! {}",
    Style::Bold.to_str(),
    Color::Cyan.to_fg_str(),
    Style::Reset.to_str()
  );

  // Example of cursor movement
  Cursor::down(2);
  Cursor::right(5);
  println!("I am here!");

  Cursor::hide();
  println!("Cursor is hidden");
  Cursor::show();
}
