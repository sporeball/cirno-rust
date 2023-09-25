use std::collections::HashMap;

use crossterm::event::KeyEvent;
use crate::terminal::KeyEventResult;

pub fn get() -> crate::project::Mode {
  crate::project::Mode {
    key_event_cb: handle_key_event,
    commands: HashMap::from([
      ('h', on_key_h as _),
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('l', on_key_l as _),
    ])
  }
}

fn handle_key_event(event: KeyEvent, state: &mut crate::CirnoState) -> KeyEventResult {
  let crossterm::event::KeyEvent { code, modifiers, kind, state: _ } = event;
  if !matches!(kind, crossterm::event::KeyEventKind::Press) {
    return KeyEventResult::Ok // TODO: return the concept of none
  }
  // exit on Ctrl+C
  if matches!(code, crossterm::event::KeyCode::Char('c')) && modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
    return KeyEventResult::Exit
  }
  if let crossterm::event::KeyCode::Char(c) = code {
    if let Some(cmd) = get().commands.get(&c) {
      return (cmd)(state);
    }
  }
  KeyEventResult::Ok
}

fn on_key_h(state: &mut crate::CirnoState) -> KeyEventResult {
  state.cursor_x -= 1;
  KeyEventResult::Ok
}

fn on_key_j(state: &mut crate::CirnoState) -> KeyEventResult {
  state.cursor_y += 1;
  KeyEventResult::Ok
}

fn on_key_k(state: &mut crate::CirnoState) -> KeyEventResult {
  state.cursor_y -= 1;
  KeyEventResult::Ok
}

fn on_key_l(state: &mut crate::CirnoState) -> KeyEventResult {
  state.cursor_x += 1;
  KeyEventResult::Ok
}