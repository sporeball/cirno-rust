use crate::{CirnoState, bar, cursor, error::CirnoError, project::{Object, ObjectEnum}};
use crossterm::style::{Color, Colors};

/// Perform a search.
pub fn query(line: String, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let mut chars = line.chars();
  clear(state)?;
  // perform query
  let result: Vec<ObjectEnum> = match chars.next() {
    Some('w') => {
      let color = chars.next();
      let label = chars.next();
      if chars.next().is_some() {
        return Err(CirnoError::InvalidSearch.into())
      }
      query_wire(color, label, state)?
    },
    Some(_) => return Err(CirnoError::InvalidSearch.into()),
    None => unreachable!(),
  };
  let len = result.len();
  if len == 0 {
    return Err(CirnoError::NoResultsFound.into())
  }
  // update state
  state.search_result.replace(result);
  for object in state.search_result.borrow().iter() {
    object.highlight(state)?;
  }
  cursor::render(state)?;
  bar::message(format!("{} results", len), state)?;
  Ok(())
}

/// Clear `state.search_result` and any associated highlighting.
pub fn clear(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  for object in state.search_result.borrow().iter() {
    object.render(Colors { foreground: None, background: None }, state)?;
  }
  state.search_result.replace(vec![]);
  Ok(())
}

/// Perform a wire search, given a color and optional label.
fn query_wire(color: Option<char>, label: Option<char>, state: &mut CirnoState) -> Result<Vec<ObjectEnum>, anyhow::Error> {
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
  let wires = binding
    .iter()
    .filter_map(|x| match x {
      ObjectEnum::Wire(w) => Some((x.to_owned(), w)),
      _ => None,
    });
  let result: Vec<ObjectEnum>;
  if let Some(l) = label {
    result = wires
      .filter(|w| w.1.color == color_struct && w.1.label == l)
      .map(|w| w.0)
      .collect();
  } else {
    result = wires
      .filter(|w| w.1.color == color_struct)
      .map(|w| w.0)
      .collect();
  }
  Ok(result)
}
