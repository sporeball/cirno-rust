use crate::{bar, project::{Object, ObjectEnum, Region, Vector2}, terminal::move_within_bounds, CirnoState};
use std::io::stdout;
use crossterm::{execute, style::Color};

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
  let (mut report, mut color) = (String::new(), Color::White);
  let objects = state.objects.borrow();
  let wire = objects
    .iter()
    .filter_map(|x| match x {
      ObjectEnum::Wire(wire) => Some(wire.to_owned()),
      _ => None,
    })
    .find(|w| {
      (state.cursor.x == w.from.x && state.cursor.y == w.from.y) ||
      (state.cursor.x == w.to.x && state.cursor.y == w.to.y)
    });
  for object in objects.iter() {
    let Some(region) = object.get_region() else { continue };
    if region.overlapping(&cursor_region) {
      (report, color) = object.report(state)?;
    }
  }
  if wire.is_some() {
    let wire = wire.unwrap();
    report = format!("({}) {}", wire.label, report);
    color = wire.color;
  }
  execute!(stdout(), crossterm::style::SetForegroundColor(color))?;
  bar::message(report, state)?;
  execute!(stdout(), crossterm::style::ResetColor)?;
  Ok(())
}
