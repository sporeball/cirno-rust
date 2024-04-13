use crate::{bar, logger, project::{Object, ObjectEnum, Region, Vector2}, terminal::move_within_bounds, CirnoState};
use std::io::stdout;
use crossterm::{execute, style::Color};

/// Move the cursor left, up to the given number of cells.
pub fn move_left(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let lim = state.cursor.x;
  if lim == 0 {
    return Ok(())
  }
  clear(state)?;
  state.cursor.x -= match cells {
    0 => 1,
    n if (1..=lim).contains(&n) => cells,
    _ => lim,
  };
  render(state)?;
  report(state)?;
  Ok(())
}

/// Move the cursor right, up to the given number of cells.
pub fn move_right(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let lim = state.meta.bounds.x - state.cursor.x - 1;
  if lim == 0 {
    return Ok(())
  }
  clear(state)?;
  state.cursor.x += match cells {
    0 => 1,
    n if (1..=lim).contains(&n) => cells,
    _ => lim,
  };
  render(state)?;
  report(state)?;
  Ok(())
}

/// Move the cursor up, up to the given number of cells.
pub fn move_up(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let lim = state.cursor.y;
  if lim == 0 {
    return Ok(())
  }
  clear(state)?;
  state.cursor.y -= match cells {
    0 => 1,
    n if (1..=lim).contains(&n) => cells,
    _ => lim,
  };
  render(state)?;
  report(state)?;
  Ok(())
}

/// Move the cursor down, up to the given number of cells.
pub fn move_down(cells: u16, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let lim = state.meta.bounds.y - state.cursor.y - 2; // ???
  if lim == 0 {
    return Ok(())
  }
  clear(state)?;
  state.cursor.y += match cells {
    0 => 1,
    n if (1..=lim).contains(&n) => cells,
    _ => lim,
  };
  render(state)?;
  report(state)?;
  Ok(())
}

/// Clear the cursor, replacing it with the character that should be underneath.
pub fn clear(state: &CirnoState) -> Result<(), anyhow::Error> {
  move_within_bounds(state.cursor.x, state.cursor.y, state)?;
  execute!(stdout(), crossterm::style::SetForegroundColor(state.char_under_cursor.1))?;
  execute!(stdout(), crossterm::style::Print(state.char_under_cursor.0))?;
  execute!(stdout(), crossterm::style::ResetColor)?;
  Ok(())
}

/// Draw the cursor.
pub fn render(state: &CirnoState) -> Result<(), anyhow::Error> {
  move_within_bounds(state.cursor.x, state.cursor.y, state)?;
  execute!(stdout(), crossterm::style::Print("@"))?;
  Ok(())
}

/// Print information about the object that the cursor is overlapping.
/// Sets `state.char_under_cursor`.
pub fn report(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  bar::clear(state)?;
  let cursor_region = Region {
    position: Vector2 { x: state.cursor.x, y: state.cursor.y },
    size: Vector2 { x: 1, y: 1 },
  };
  let (mut report, mut color) = (String::new(), Color::White);
  let (mut u_char, mut u_color) = (' ', Color::White);
  let objects = state.objects.borrow();
  let wire = objects
    .iter()
    .filter_map(|x| match x {
      ObjectEnum::Wire(wire) => Some(wire.to_owned()),
      _ => None,
    })
    .find(|w| w.is_connected_to(state.cursor));
  for object in objects.iter() {
    let Some(region) = object.get_region() else { continue };
    if region.overlapping(&cursor_region) {
      (report, color) = object.report(state)?;
      (u_char, u_color) = object.get_char().unwrap();
    }
  }
  if wire.is_some() {
    let wire = wire.unwrap();
    report = format!("({}) {}", wire.label, report);
    color = wire.color;
    (u_char, u_color) = wire.get_char().unwrap();
  }
  // set state.char_under_cursor
  state.char_under_cursor = (u_char, u_color);
  // print report
  execute!(stdout(), crossterm::style::SetForegroundColor(color))?;
  bar::message(report, state)?;
  execute!(stdout(), crossterm::style::ResetColor)?;
  Ok(())
}

/// Print information about the object that the cursor is overlapping to the console.
pub fn debug(state: &CirnoState) -> Result<(), anyhow::Error> {
  let cursor_region = Region {
    position: Vector2 { x: state.cursor.x, y: state.cursor.y },
    size: Vector2 { x: 1, y: 1 },
  };
  let objects = state.objects.borrow();
  let wire = objects
    .iter()
    .filter_map(|x| match x {
      ObjectEnum::Wire(wire) => Some(wire.to_owned()),
      _ => None,
    })
    .find(|w| w.is_connected_to(state.cursor));
  for object in objects.iter() {
    let Some(region) = object.get_region() else { continue };
    if region.overlapping(&cursor_region) {
      logger::debug(format!("{:?} -> {:?}", state.cursor, object));
    }
  }
  if wire.is_some() {
    let wire = wire.unwrap();
    logger::debug(format!("w: {:?}", wire));
  }
  Ok(())
}
