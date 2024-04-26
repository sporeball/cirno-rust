use crate::{open, bar, cursor, error::{CirnoError, try_to}, project::{Mode, Modes}, search, terminal::{backspace, clear_all, move_to, read_line, EventResult}, CirnoState};
use std::collections::HashMap;
use std::io::stdout;
use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::execute;

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
      ('C', on_key_cap_c as _),
      ('L', on_key_cap_l as _),
      (':', on_key_colon as _),
      ('/', on_key_slash as _),
    ]),
    commands: HashMap::from([
      // ("open".to_string(), command_open as _),
      ("q".to_string(), command_q as _),
      ("splash".to_string(), splash as _),
    ]),
    arg_commands: HashMap::from([
      ("open".to_string(), command_open as _),
    ]),
  }
}

pub fn splash(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if state.columns < 30 || state.rows < 6 {
    return Ok(EventResult::Ok)
  }
  let center_x = state.columns / 2;
  let center_y = state.rows / 2;
  clear_all()?;
  move_to(center_x - 12, center_y - 2)?;
  execute!(stdout(), crossterm::style::Print("\u{1b}[36mcirno\u{1b}[0m"))?;
  move_to(center_x - 12, center_y - 1)?;
  execute!(stdout(), crossterm::style::Print("\u{1b}[90m\"I can go anywhere\u{1b}[0m"))?;
  move_to(center_x - 12, center_y)?;
  execute!(stdout(), crossterm::style::Print("\u{1b}[90mand do anything I want!\"\u{1b}[0m"))?;
  move_to(center_x - 15, center_y + 2)?;
  execute!(stdout(), crossterm::style::Print("type :open \u{1b}[34m<filename>\u{1b}[0m to start"))?;
  move_to(center_x - 15, center_y + 3)?;
  execute!(stdout(), crossterm::style::Print("type :q\u{1b}[34m<Enter>\u{1b}[0m to quit"))?;
  Ok(EventResult::Ok)
}

fn on_mode_set(state: &mut CirnoState) -> Result<(), anyhow::Error> {
  clear_all()?;
  if state.project.is_none() {
    splash(state)?;
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
  if state.repeat_amount == 0 {
    state.repeat_amount = digit;
  } else if state.repeat_amount < 1000 {
    state.repeat_amount *= 10;
    state.repeat_amount += digit;
  } else {
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

fn on_key_cap_c(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  state.set_mode(Modes::Console)?;
  Ok(EventResult::Ok)
}

fn on_key_cap_l(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  cursor::debug(state)?;
  Ok(EventResult::Ok)
}

fn on_key_colon(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  let mode = get();
  bar::message(":".to_string(), state)?;
  let line = read_line()?;
  // remove the colon if the command comes back empty
  if line.eq("") {
    backspace()?;
  } else if let Some(cmd) = mode.commands.get(&line) {
    return (cmd)(state);
  } else if let Some(key) = mode.arg_commands.keys().find(|&k| line.starts_with(k)) {
    let cmd = mode.arg_commands.get(key).unwrap();
    let args = line.split(' ').collect::<Vec<&str>>();
    return (cmd)(args, state);
  } else {
    return Err(CirnoError::InvalidCommand(line).into());
  }
  Ok(EventResult::Drop)
}

fn on_key_slash(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  bar::message("/".to_string(), state)?;
  let line = read_line()?;
  // remove the slash if the search comes back empty
  if line.eq("") {
    backspace()?;
  } else {
    search::query(line, state)?;
  }
  Ok(EventResult::Drop)
}

fn command_q(_state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  Ok(EventResult::Exit)
}

fn command_open(args: Vec<&str>, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  if let [_, filename] = args.as_slice() {
    try_to(open(PathBuf::from(filename), state), state)?;
    return Ok(EventResult::Ok)
  }
  Err(CirnoError::ArgumentError(1, args.len() - 1).into())
}
