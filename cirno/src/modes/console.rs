use crate::{CirnoState, project::{Mode, Modes}, terminal::{EventResult, clear_all, move_to, println}};
use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn get() -> Mode {
  Mode {
    mode_set_cb: on_mode_set,
    key_event_cb,
    resize_event_cb: handle_resize_event,
    key_commands: HashMap::from([
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('C', on_key_cap_c as _),
    ]),
  }
}

fn on_mode_set(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear_all()?;
  move_to(0, 0)?;
  // log all items in LOG_STATE
  let log = crate::logger::LOG_STATE.read().unwrap();
  for item in log.iter() {
    println(item.lines.join("\r\n").as_str(), state)?;
  }
  Ok(())
}

fn key_event_cb(_code: KeyCode, _modifiers: KeyModifiers, _state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  Ok(EventResult::Drop)
}

fn handle_resize_event(_state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  Ok(EventResult::Ok)
}

fn on_key_j(_state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  // TODO
  Ok(EventResult::Ok)
}

fn on_key_k(_state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  // TODO
  Ok(EventResult::Ok)
}

fn on_key_cap_c(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  state.set_mode(Modes::Normal)?;
  Ok(EventResult::Ok)
}
