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
  let cursor_region = Region {
    position: Vector2 { x: state.cursor.x, y: state.cursor.y },
    size: Vector2 { x: 1, y: 1 },
  };
  let objects_binding = state.objects.borrow();
  let object_regions = objects_binding
    .iter()
    .map(|x| match x.to_owned() {
      Object::Chip(chip) => Some(chip.region),
      Object::Meta(_meta) => None,
      Object::Net(net) => Some(net.region),
      Object::Pin(pin) => Some(pin.region),
    });
  for (index, region) in object_regions.enumerate() {
    if region.is_some_and(|x| x.overlapping(cursor_region.to_owned())) {
      let object = objects_binding.get(index).unwrap();
      let report = object.report(state)?;
      bar::message(format!("{}", report), state)?;
      return Ok(())
    }
  }
  bar::clear(state)?;
  Ok(())
}