use crate::{CirnoState, project::{Mode, Modes}, terminal::{EventResult, clear_all, move_to, println}};
use std::collections::HashMap;
use crossterm::event::KeyEvent;

pub fn get() -> Mode {
  Mode {
    mode_set_cb: on_mode_set,
    key_event_cb: handle_key_event,
    resize_event_cb: handle_resize_event,
    key_commands: HashMap::from([
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('C', on_key_cap_c as _),
    ]),
    commands: HashMap::from([]),
    arg_commands: HashMap::from([]),
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

fn handle_key_event(event: KeyEvent, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  let crossterm::event::KeyEvent { code, modifiers: _, kind, state: _ } = event;
  if !matches!(kind, crossterm::event::KeyEventKind::Press) {
    return Ok(EventResult::Drop)
  }
  if let crossterm::event::KeyCode::Char(c) = code {
    if let Some(cmd) = get().key_commands.get(&c) {
      return (cmd)(state);
    }
  }
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
