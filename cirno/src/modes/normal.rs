use crate::{bar, command::{self, Command, Splash}, cursor, error::{CirnoError, try_to}, project::{Mode, Modes}, search, terminal::{clear_all, read_key_presses, EventResult}, CirnoState};
use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn get() -> Mode {
  Mode {
    mode_set_cb: on_mode_set,
    key_event_cb,
    resize_event_cb: handle_resize_event,
    key_commands: HashMap::from([
      ('h', on_key_h as _),
      ('j', on_key_j as _),
      ('k', on_key_k as _),
      ('l', on_key_l as _),
      ('p', on_key_p as _),
      ('C', on_key_cap_c as _),
      ('L', on_key_cap_l as _),
      (':', on_key_colon as _),
      ('/', on_key_slash as _),
    ]),
  }
}

fn on_mode_set(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear_all()?;
  if state.project.is_none() {
    Splash(Vec::new()).execute(state)?;
    return Ok(())
  }
  state.verify()?;
  state.render()?;
  Ok(())
}

fn key_event_cb(code: KeyCode, modifiers: KeyModifiers, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if let KeyCode::Char(c) = code {
    // Ctrl+C
    if c == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
      bar::message("type  :q  and press <Enter> to exit cirno".to_string(), state)?;
    }
    if let '0' ..= '9' = c {
      try_to(update_repeat_amount(c, state), state)?;
    }
  }
  Ok(EventResult::Drop)
}

fn handle_resize_event(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  clear_all()?;
  state.verify_size()?;
  state.render()?;
  Ok(EventResult::Ok)
}

/// Update `state.repeat_amount` based on a character `c`.
/// This function does not verify that `c` is a number.
fn update_repeat_amount(c: char, state: &mut CirnoState) -> Result<(), CirnoError> {
  let digit = c.to_digit(10).unwrap() as u16;
  // update
  if state.repeat_amount == 0 {
    state.repeat_amount = digit;
  } else {
    state.repeat_amount *= 10;
    state.repeat_amount += digit;
  }
  // limit
  if state.repeat_amount > 1000 {
    state.repeat_amount = 0;
    return Err(CirnoError::TooManyRepetitions)
  }
  Ok(())
}

fn on_key_h(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  cursor::move_left(state.repeat_amount, state)?;
  Ok(EventResult::Ok)
}

fn on_key_j(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  cursor::move_down(state.repeat_amount, state)?;
  Ok(EventResult::Ok)
}

fn on_key_k(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  cursor::move_up(state.repeat_amount, state)?;
  Ok(EventResult::Ok)
}

fn on_key_l(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  cursor::move_right(state.repeat_amount, state)?;
  Ok(EventResult::Ok)
}

fn on_key_p(_state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if let Some(sequence) = read_key_presses(3)? {
    crate::logger::debug(format!("{:?}", sequence));
  }
  Ok(EventResult::Ok)
}

fn on_key_cap_c(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  state.set_mode(Modes::Console)?;
  Ok(EventResult::Ok)
}

fn on_key_cap_l(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  cursor::debug(state)?;
  Ok(EventResult::Ok)
}

fn on_key_colon(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  return command::read_from_bar(state)
}

fn on_key_slash(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  return search::read_from_bar(state)
}
