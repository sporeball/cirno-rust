use crate::{CirnoState, bar, project::{Mode, Modes}, terminal::{EventResult, backspace, clear_all, read_line}};
use std::collections::HashMap;
use crossterm::event::KeyEvent;

pub fn get() -> Mode {
  Mode {
    mode_set_cb: on_mode_set,
    key_event_cb: handle_key_event,
    resize_event_cb: handle_resize_event,
    key_commands: HashMap::from([
      ('h', on_key_h as _),
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('l', on_key_l as _),
      ('C', on_key_cap_c as _),
      (':', on_key_colon as _),
    ]),
    commands: HashMap::from([
      ("q".to_string(), command_q as _),
    ]),
  }
}

fn on_mode_set(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear_all()?;
  state.verify()?;
  state.render()?;
  Ok(())
}

fn handle_key_event(event: KeyEvent, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  let crossterm::event::KeyEvent { code, modifiers, kind, state: _ } = event;
  if !matches!(kind, crossterm::event::KeyEventKind::Press) {
    return Ok(EventResult::Drop)
  }
  // Ctrl+C
  if matches!(code, crossterm::event::KeyCode::Char('c')) && modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
    bar::message("type  :q  and press <Enter> to exit cirno".to_string(), state)?;
    return Ok(EventResult::Ok)
  }
  if let crossterm::event::KeyCode::Char(c) = code {
    if let Some(cmd) = get().key_commands.get(&c) {
      return (cmd)(state);
    }
  }
  Ok(EventResult::Drop)
}

fn handle_resize_event(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  clear_all()?;
  state.verify()?;
  state.render()?;
  Ok(EventResult::Ok)
}

fn on_key_h(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if state.cursor.x > 0 {
    state.cursor.x -= 1;
    state.render()?;
  }
  Ok(EventResult::Ok)
}

fn on_key_j(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  state.cursor.y += 1;
  state.render()?;
  Ok(EventResult::Ok)
}

fn on_key_k(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if state.cursor.y > 0 {
    state.cursor.y -= 1;
    state.render()?;
  }
  Ok(EventResult::Ok)
}

fn on_key_l(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  state.cursor.x += 1;
  state.render()?;
  Ok(EventResult::Ok)
}

fn on_key_cap_c(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  state.set_mode(Modes::Console)?;
  Ok(EventResult::Ok)
}

fn on_key_colon(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  bar::message(":".to_string(), state)?;
  let command = read_line()?;
  // remove the colon if the command comes back empty
  if command.eq("") {
    backspace()?;
    return Ok(EventResult::Drop)
  }
  if let Some(cmd) = get().commands.get(&command) {
    return (cmd)(state);
  } else {
    return Ok(EventResult::Drop)
  }
}

fn command_q(_state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  Ok(EventResult::Exit)
}
