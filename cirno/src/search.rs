use crate::{CirnoState, bar, error::CirnoError, project::{Object, ObjectEnum, Wire}};
use crossterm::style::Color;

pub fn query(line: String, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let mut chars = line.chars();
  match chars.next() {
    Some('w') => {
      let color = chars.next();
      let label = chars.next();
      query_wire(color, label, state)?;
    },
    Some(_) => return Err(CirnoError::InvalidSearch.into()),
    None => unreachable!(),
  };
  if chars.next().is_some() {
    return Err(CirnoError::InvalidSearch.into())
  }
  Ok(())
}

// TODO: result cannot be cleared unless you move over the highlighted parts
fn query_wire(color: Option<char>, label: Option<char>, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let color_struct = match color {
    Some('r') => Color::Red,
    Some('g') => Color::Green,
    Some('y') => Color::Yellow,
    Some('b') => Color::Blue,
    Some('m') => Color::Magenta,
    Some('c') => Color::Cyan,
    Some(_) => return Err(CirnoError::InvalidSearch.into()),
    None => return Err(CirnoError::InvalidSearch.into()),
  };
  let binding = state.objects.borrow();
  let mut wires = binding
    .iter()
    .filter_map(|x| match x {
      ObjectEnum::Wire(wire) => Some(wire),
      _ => None,
    });
  if let Some(l) = label {
    let found = wires.find(|w| w.color == color_struct && w.label == l);
    if found.is_none() {
      return Err(CirnoError::NoResultsFound.into())
    }
    let found = found.unwrap();
    found.highlight(state)?;
    bar::message("1 result".to_string(), state)?;
  } else {
    let found: Vec<&Wire> = wires.filter(|w| w.color == color_struct).collect();
    let len = found.len();
    if len == 0 {
      return Err(CirnoError::NoResultsFound.into())
    }
    for object in found {
      object.highlight(state)?;
    }
    bar::message(format!("{} results", len), state)?;
  }
  Ok(())
}
