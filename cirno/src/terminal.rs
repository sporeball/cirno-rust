use crate::{CirnoState};
use std::io;
use std::io::stdout;
use crossterm::execute;

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

pub fn is_out_of_bounds(x: u16, y: u16, state: &CirnoState) -> bool {
  if x > state.bound_x - 1 || y > state.bound_y - 1 { // zero
    return true
  }
  false
}
