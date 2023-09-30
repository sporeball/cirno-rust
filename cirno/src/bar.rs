use crate::CirnoState;
use std::io;
use std::io::stdout;
use crossterm::execute;

pub fn clear(state: &CirnoState) -> Result<(), io::Error> {
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - 1))?;
  execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))?;
  Ok(())
}

pub fn message(msg: String, state: &CirnoState) -> Result<(), io::Error> {
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - 1))?;
  clear(state)?;
  execute!(stdout(), crossterm::style::Print(msg.to_string()))?;
  Ok(())
}
