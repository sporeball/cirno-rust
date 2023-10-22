use crate::{CirnoState, parser::Token};
use std::io::stdout;
use std::fmt::Display;
use crossterm::execute;

#[derive(thiserror::Error, Debug)]
pub enum CirnoError {
  #[error("could not open project")]
  CouldNotOpenProject,
  #[error("invalid attribute '{0}'")]
  InvalidAttribute(String),
  #[error("attribute '{0}' is invalid for {1} objects")]
  InvalidAttributeForObject(String, String),
  #[error("invalid filetype '{0}'")]
  InvalidFiletype(String),
  #[error("invalid object type '{0}'")]
  InvalidObjectType(String),
  #[error("invalid value attribute: {0}")]
  InvalidValueAttribute(String),
  #[error("invalid value '{0}' for attribute '{1}'")]
  InvalidValueForAttribute(String, String),
  #[error("missing attribute: {0}")]
  MissingAttribute(String),
  #[error("missing meta object")]
  MissingMetaObject,
  #[error("invalid value for attribute '{0}'")]
  NamelessInvalidValueForAttribute(String),
  #[error("opening .cic files is not yet implemented")]
  OpenCicNotImplemented,
  #[error("object out of bounds")]
  OutOfBounds,
  #[error("expected {0}, got None")]
  OutOfTokens(Token),
  #[error("expected Number, got None")]
  OutOfTokensExpectedNumber,
  #[error("terminal too small")]
  TerminalTooSmall,
  #[error("expected {0}, got {1}")]
  UnexpectedToken(Token, Token),
  #[error("expected Number, got {0}")]
  UnexpectedTokenExpectedNumber(Token),
  #[error("unrecognized token")] // TODO: add a field
  UnrecognizedToken,
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
