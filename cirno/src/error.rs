use crate::{CirnoState, logger, parser::Token, project::Attribute};
use std::io::stdout;
use std::fmt::Display;
use crossterm::execute;

#[derive(thiserror::Error, Debug)]
pub enum CirnoError {
  #[error("invalid attribute '{0}'")]
  InvalidAttribute(String),
  #[error("attribute '{0}' is invalid for {1} objects")]
  InvalidAttributeForObject(Attribute, String),
  #[error("invalid color attribute: {0}")]
  InvalidColorAttribute(String),
  #[error("invalid file '{0}'")]
  InvalidFile(String),
  #[error("invalid filetype '{0}'")]
  InvalidFiletype(String),
  #[error("invalid object type '{0}'")]
  InvalidObjectType(String),
  #[error("invalid value attribute: {0}")]
  InvalidValueAttribute(String),
  #[error("invalid value '{0}' for attribute '{1}'")]
  InvalidValueForAttribute(String, String),
  #[error("meta object missing or invalid")]
  MetaObjectError,
  #[error("missing attribute: {0}")]
  MissingAttribute(String),
  #[error("invalid value for attribute '{0}'")]
  NamelessInvalidValueForAttribute(String),
  #[error("'{0}' not found in stdlib")]
  NotFoundInStdlib(String),
  #[error("opening .cic files is not yet implemented")]
  OpenCicNotImplemented,
  #[error("object out of bounds")]
  OutOfBounds,
  #[error("expected {0}, got None")]
  OutOfTokens(Token),
  #[error("expected Number, got None")]
  OutOfTokensExpectedNumber,
  #[error("regions {0} and {1} are overlapping")]
  OverlappingRegion(usize, usize),
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
      state.error = e.to_string();
      throw(state)?;
      Ok(None)
    },
  }
}

pub fn throw(state: &CirnoState) -> Result<(), anyhow::Error> {
  logger::error(state.error.clone());
  display(state)?;
  Ok(())
}

pub fn display(state: &CirnoState) -> Result<(), anyhow::Error> {
  clear(state)?;
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - 1))?;
  execute!(stdout(), crossterm::style::SetBackgroundColor(crossterm::style::Color::Red))?;
  execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::White))?;
  execute!(stdout(), crossterm::style::Print(format!("E: {}", state.error)))?;
  execute!(stdout(), crossterm::style::ResetColor)?;
  Ok(())
}

pub fn clear(state: &CirnoState) -> Result<(), anyhow::Error> {
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - 1))?;
  execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))?;
  Ok(())
}
