use crate::{CirnoState, bar, logger, project::Mode, terminal::{KeyEventResult, backspace, read_line}};
use std::collections::HashMap;
use std::io;
use crossterm::event::KeyEvent;

pub fn get() -> Mode {
  Mode {
    key_event_cb: handle_key_event,
    commands: HashMap::from([
      ('h', on_key_h as _),
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('l', on_key_l as _),
      (':', on_key_colon as _),
    ])
  }
}

fn handle_key_event(event: KeyEvent, state: &mut CirnoState) -> KeyEventResult {
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
      return (cmd)(state).unwrap();
    }
  }
  KeyEventResult::Ok
}

fn on_key_h(state: &mut CirnoState) -> Result<KeyEventResult, io::Error> {
  if state.cursor_x > 0 {
    state.cursor_x -= 1;
    state.render()?;
  }
  Ok(KeyEventResult::Ok)
}

fn on_key_j(state: &mut CirnoState) -> Result<KeyEventResult, io::Error> {
  state.cursor_y += 1;
  state.render()?;
  Ok(KeyEventResult::Ok)
}

fn on_key_k(state: &mut CirnoState) -> Result<KeyEventResult, io::Error> {
  if state.cursor_y > 0 {
    state.cursor_y -= 1;
    state.render()?;
  }
  Ok(KeyEventResult::Ok)
}

fn on_key_l(state: &mut CirnoState) -> Result<KeyEventResult, io::Error> {
  state.cursor_x += 1;
  state.render()?;
  Ok(KeyEventResult::Ok)
}

fn on_key_colon(state: &mut CirnoState) -> Result<KeyEventResult, io::Error> {
  bar::message(":".to_string(), state)?;
  let command = read_line()?;
  // remove the colon if the command comes back empty
  // TODO: removed if Enter is pressed, but should be kept as in vim
  if command.eq("") {
    backspace()?;
  }
  logger::debug(&command);
  Ok(KeyEventResult::Ok)
}
