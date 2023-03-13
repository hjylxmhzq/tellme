use termimad::MadSkin;
use termimad::*;

pub fn format_print(text: &str) -> String {
  let mut skin = MadSkin::default();
  skin.set_headers_fg(rgb(255, 187, 0));
  skin.bold.set_fg(crossterm::style::Color::Yellow);
  skin.italic.set_fgbg(crossterm::style::Color::Magenta, rgb(30, 30, 40));
  skin.paragraph.align = Alignment::Left;
  skin.table.align = Alignment::Left;
  skin.term_text(&text).to_string()
}