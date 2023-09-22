use std::io;
use crossterm::execute;

pub enum KeyEventResult {
  Ok,
  Err,
  Exit,
}

pub fn enter() {
  crossterm::terminal::enable_raw_mode();
  execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen);
  execute!(io::stdout(), crossterm::terminal::DisableLineWrap);
  execute!(io::stdout(), crossterm::cursor::Hide);
}

pub fn exit() {
  execute!(io::stdout(), crossterm::cursor::Show);
  execute!(io::stdout(), crossterm::terminal::EnableLineWrap);
  execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
  crossterm::terminal::disable_raw_mode();
}
