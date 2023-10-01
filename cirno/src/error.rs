use crate::{CirnoState, bar};
use std::io;
use std::io::stdout;
use std::error::Error;
use std::fmt::Display;
use crossterm::execute;
use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum CirnoError {
  #[error("bounds not set")]
  BoundsNotSet,
  #[error("missing meta object")]
  MissingMetaObject,
}

pub fn try_to<T, E: Display>(f: Result<T, E>, state: &mut CirnoState) -> Result<Option<T>, anyhow::Error> {
  match f {
    Ok(v) => Ok(Some(v)),
    Err(e) => {
      state.errors.push(e.to_string());
      display(state);
      Ok(None)
    },
  }
}

pub fn display(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let l: u16 = state.errors.len() as u16;
  execute!(stdout(), crossterm::cursor::MoveTo(0, state.rows - l - 1))?;
  execute!(stdout(), crossterm::style::SetBackgroundColor(crossterm::style::Color::Red))?;
  execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::White))?;
  for e in state.errors.clone() {
    execute!(stdout(), crossterm::cursor::MoveToNextLine(1))?;
    execute!(stdout(), crossterm::style::Print(format!("E: {e}")))?;
  }
  execute!(stdout(), crossterm::style::ResetColor)?;
  Ok(())
}
