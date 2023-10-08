use crate::CirnoState;
use std::io::stdout;
use std::fmt::Display;
use crossterm::execute;

#[derive(thiserror::Error, Debug)]
pub enum CirnoError {
  #[error("bounds not set")]
  BoundsNotSet,
  #[error("could not open project")]
  CouldNotOpenProject,
  #[error("could not apply {0} to chip")]
  InvalidChipAttribute(String),
  #[error("invalid filetype: {0}")]
  InvalidFiletype(String),
  #[error("could not apply {0} to meta object")]
  InvalidMetaAttribute(String),
  #[error("could not apply {0} to net")]
  InvalidNetAttribute(String),
  #[error("could not apply {0} to pin")]
  InvalidPinAttribute(String),
  #[error("missing meta object")]
  MissingMetaObject,
  #[error("opening .cic files is not yet implemented")]
  OpenCicNotImplemented,
  #[error("object out of bounds")]
  OutOfBounds,
  #[error("terminal too small")]
  TerminalTooSmall,
}

/// Call a function, possibly producing a recoverable error message as a side effect.
pub fn try_to<T, E: Display>(f: Result<T, E>, state: &mut CirnoState) -> Result<Option<T>, anyhow::Error> {
  match f {
    Ok(v) => Ok(Some(v)),
    Err(e) => {
      throw(e.to_string(), state)?;
      Ok(None)
    },
  }
}

pub fn throw(e: String, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  state.errors.push(e);
  display(state)?;
  Ok(())
}

pub fn display(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear(state)?;
  let l: u16 = state.errors.len() as u16;
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - l - 1))?;
  execute!(stdout(), crossterm::style::SetBackgroundColor(crossterm::style::Color::Red))?;
  execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::White))?;
  for e in state.errors.clone() {
    execute!(stdout(), crossterm::cursor::MoveToNextLine(1))?;
    execute!(stdout(), crossterm::style::Print(format!("E: {}", e)))?;
  }
  execute!(stdout(), crossterm::style::ResetColor)?;
  Ok(())
}

pub fn clear(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let l: u16 = state.errors.len() as u16;
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - l - 1))?;
  for _e in state.errors.clone() {
    execute!(stdout(), crossterm::cursor::MoveToNextLine(1))?;
    execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))?;
  }
  Ok(())
}
