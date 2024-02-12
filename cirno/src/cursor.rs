use crate::{CirnoState, terminal::move_within_bounds};
use std::io::stdout;
use crossterm::execute;

pub fn move_left(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear(state)?;
  state.cursor.x -= cells;
  Ok(())
}

pub fn move_right(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear(state)?;
  state.cursor.x += cells;
  Ok(())
}

pub fn move_up(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear(state)?;
  state.cursor.y -= cells;
  Ok(())
}

pub fn move_down(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear(state)?;
  state.cursor.y += cells;
  Ok(())
}

pub fn clear(state: &CirnoState) -> Result<(), anyhow::Error> {
  move_within_bounds(state.cursor.x, state.cursor.y, state)?;
  execute!(stdout(), crossterm::style::Print(" "))?;
  Ok(())
}

pub fn render(state: &CirnoState) -> Result<(), anyhow::Error> {
  move_within_bounds(state.cursor.x, state.cursor.y, state)?;
  execute!(stdout(), crossterm::style::Print("@"))?;
  Ok(())
}