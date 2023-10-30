use crate::{CirnoState, modes::switch_to_mode, project::{Mode, Modes}, terminal::KeyEventResult};
use std::collections::HashMap;
use crossterm::event::KeyEvent;

pub fn get() -> Mode {
  Mode {
    key_event_cb: handle_key_event,
    key_commands: HashMap::from([
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('C', on_key_cap_c as _),
    ]),
    commands: HashMap::from([]),
  }
}

fn handle_key_event(event: KeyEvent, state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  let crossterm::event::KeyEvent { code, modifiers, kind, state: _ } = event;
  if !matches!(kind, crossterm::event::KeyEventKind::Press) {
    return Ok(KeyEventResult::Ok) // TODO: return the concept of none
  }
  if let crossterm::event::KeyCode::Char(c) = code {
    if let Some(cmd) = get().key_commands.get(&c) {
      return (cmd)(state);
    }
  }
  Ok(KeyEventResult::Ok)
}

fn on_key_j(state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  // TODO
  Ok(KeyEventResult::Ok)
}

fn on_key_k(state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  // TODO
  Ok(KeyEventResult::Ok)
}

fn on_key_cap_c(state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  switch_to_mode(Modes::Normal, state);
  Ok(KeyEventResult::Ok)
}
