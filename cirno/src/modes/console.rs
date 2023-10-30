use crate::{CirnoState, project::{Mode, Modes}, terminal::{KeyEventResult, clear_all, move_to, println}};
use std::collections::HashMap;
use crossterm::event::KeyEvent;

pub fn get() -> Mode {
  Mode {
    mode_set_cb: on_mode_set,
    key_event_cb: handle_key_event,
    key_commands: HashMap::from([
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('C', on_key_cap_c as _),
    ]),
    commands: HashMap::from([]),
  }
}

fn on_mode_set(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear_all()?;
  move_to(0, 0)?;
  // log all items in LOG_STATE
  let log = crate::logger::LOG_STATE.read().unwrap();
  for item in log.iter() {
    for line in &item.lines {
      println(line, state)?;
    }
  }
  Ok(())
}

fn handle_key_event(event: KeyEvent, state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  let crossterm::event::KeyEvent { code, modifiers: _, kind, state: _ } = event;
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

fn on_key_j(_state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  // TODO
  Ok(KeyEventResult::Ok)
}

fn on_key_k(_state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  // TODO
  Ok(KeyEventResult::Ok)
}

fn on_key_cap_c(state: &mut CirnoState) -> Result<KeyEventResult, anyhow::Error> {
  state.set_mode(Modes::Normal)?;
  Ok(KeyEventResult::Ok)
}
