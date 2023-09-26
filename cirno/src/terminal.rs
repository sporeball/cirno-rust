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

pub fn is_y_offscreen_relative_to_center(y: u16, state: &CirnoState) -> Result<bool, io::Error> {
  let center_y = state.rows / 2;
  let visual_y = center_y + y - state.cursor_y;
  if visual_y > state.rows {
    return Ok(true)
  }
  Ok(false)
}

pub fn is_offscreen_relative_to_center(x: u16, y: u16, state: &CirnoState) -> Result<bool, io::Error> {
  let center_x = state.columns / 2;
  let center_y = state.rows / 2;
  let visual_x = center_x + x - state.cursor_x;
  let visual_y = center_y + y - state.cursor_y;
  if visual_x > state.columns || visual_y > state.rows {
    return Ok(true)
  }
  Ok(false)
}

pub fn set_x_relative_to_center(x: u16, state: &CirnoState) -> Result<(), io::Error> {
  let center_x = state.columns / 2;
  let visual_x = x + center_x - state.cursor_x - 1;
  execute!(stdout(), crossterm::cursor::MoveToColumn(visual_x))?;
  Ok(())
}

pub fn set_y_relative_to_center(y: u16, state: &CirnoState) -> Result<(), io::Error> {
  let center_y = state.rows / 2;
  let visual_y = y + center_y - state.cursor_y - 1;
  execute!(stdout(), crossterm::cursor::MoveToRow(visual_y))?;
  Ok(())
}

pub fn move_relative_to_center(x: u16, y: u16, state: &CirnoState) -> Result<(), io::Error> {
  set_x_relative_to_center(x, state)?;
  set_y_relative_to_center(y, state)?;
  Ok(())
}
