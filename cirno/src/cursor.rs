use crate::{bar, project::{Object, Region, Vector2}, terminal::move_within_bounds, CirnoState};
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

pub fn report(state: &CirnoState) -> Result<(), anyhow::Error> {
  bar::clear(state)?;
  let cursor_region = Region {
    position: Vector2 { x: state.cursor.x, y: state.cursor.y },
    size: Vector2 { x: 1, y: 1 },
  };
  for object in state.objects.borrow().iter() {
    let Some(region) = object.get_region() else { continue };
    if region.overlapping(&cursor_region) {
      let (report, color) = object.report(state)?;
      execute!(stdout(), crossterm::style::SetForegroundColor(color))?;
      bar::message(report, state)?;
      execute!(stdout(), crossterm::style::ResetColor)?;
    }
  }
  Ok(())
}
