use crate::{CirnoState, bar, command::{self, Command, Splash}, project::Mode, terminal::{EventResult, clear_all}};
use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn get() -> Mode {
  Mode {
    mode_set_cb: on_mode_set,
    key_event_cb,
    resize_event_cb: handle_resize_event,
    key_commands: HashMap::from([
      (':', on_key_colon as _),
    ]),
  }
}

fn on_mode_set(_state: &mut CirnoState) -> Result<(), anyhow::Error> {
  Ok(())
}

fn key_event_cb(code: KeyCode, modifiers: KeyModifiers, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if let KeyCode::Char(c) = code {
    // TODO: this is repeated...
    if c == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
      bar::message("type  :q  and press <Enter> to exit cirno".to_string(), state)?;
    }
  }
  Ok(EventResult::Drop)
}

fn handle_resize_event(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  clear_all()?;
  Splash(Vec::new()).execute(state)?;
  Ok(EventResult::Ok)
}

fn on_key_colon(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  return command::read_from_bar(state)
}
