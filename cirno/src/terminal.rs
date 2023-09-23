use std::io;
use crossterm::execute;
// use crossterm::event::{Event, KeyEvent};

pub enum KeyEventResult {
  Ok,
  Err,
  Exit,
}

pub fn enter() -> Result<(), io::Error> {
  crossterm::terminal::enable_raw_mode()?;
  execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
  execute!(io::stdout(), crossterm::terminal::DisableLineWrap)?;
  execute!(io::stdout(), crossterm::cursor::Hide)?;
  Ok(())
}

pub fn exit() -> Result<(), io::Error> {
  execute!(io::stdout(), crossterm::cursor::Show)?;
  execute!(io::stdout(), crossterm::terminal::EnableLineWrap)?;
  execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}