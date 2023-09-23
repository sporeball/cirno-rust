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

fn handle_key_event(event: KeyEvent, context: &mut crate::modes::CirnoContext) -> KeyEventResult {
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
      return (cmd)(context);
    }
  }
  KeyEventResult::Ok
}

fn on_key_h(context: &mut crate::modes::CirnoContext) -> KeyEventResult {
  context.cursor_x -= 1;
  KeyEventResult::Ok
}

fn on_key_j(context: &mut crate::modes::CirnoContext) -> KeyEventResult {
  context.cursor_y -= 1;
  KeyEventResult::Ok
}

fn on_key_k(context: &mut crate::modes::CirnoContext) -> KeyEventResult {
  context.cursor_y += 1;
  KeyEventResult::Ok
}

fn on_key_l(context: &mut crate::modes::CirnoContext) -> KeyEventResult {
  context.cursor_x += 1;
  KeyEventResult::Ok
}