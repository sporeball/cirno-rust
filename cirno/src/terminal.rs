use crate::{CirnoState, error::CirnoError};
use std::io;
use std::io::stdout;
use crossterm::{event::{Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::ClearType};

pub enum EventResult {
  Drop,
  Err,
  Exit,
  Ok
}

pub fn println(s: &str, state: &CirnoState) -> Result<(), io::Error> {
  let (_, row) = crossterm::cursor::position()?;
  if row == state.rows - 1 {
    execute!(stdout(), crossterm::terminal::ScrollUp(1))?;
  }
  execute!(stdout(), crossterm::style::Print(s))?;
  execute!(stdout(), crossterm::cursor::MoveToNextLine(1))?;
  Ok(())
}

pub fn clear_all() -> Result<(), io::Error> {
  execute!(stdout(), crossterm::terminal::Clear(ClearType::All))?;
  Ok(())
}

pub fn enter() -> Result<(), io::Error> {
  crossterm::terminal::enable_raw_mode()?;
  execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
  // execute!(stdout(), crossterm::terminal::DisableLineWrap)?;
  execute!(stdout(), crossterm::cursor::Hide)?;
  Ok(())
}

pub fn exit() -> Result<(), io::Error> {
  execute!(stdout(), crossterm::cursor::Show)?;
  // execute!(stdout(), crossterm::terminal::EnableLineWrap)?;
  execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
  crossterm::terminal::disable_raw_mode()?;
  Ok(())
}

pub fn backspace() -> Result<(), io::Error> {
  execute!(stdout(), crossterm::cursor::MoveLeft(1))?;
  execute!(stdout(), crossterm::style::Print(" "))?;
  execute!(stdout(), crossterm::cursor::MoveLeft(1))?;
  Ok(())
}

pub fn move_to(x: u16, y: u16) -> Result<(), io::Error> {
  execute!(stdout(), crossterm::cursor::MoveTo(x, y))?;
  Ok(())
}

pub fn move_within_bounds(x: u16, y: u16, state: &CirnoState) -> Result<(), io::Error> {
  let bound_x = state.meta.bounds.x;
  let bound_y = state.meta.bounds.y;
  let center_x = state.columns / 2;
  let center_y = state.rows / 2;
  let min_x = center_x - (bound_x / 2);
  let min_y = center_y - (bound_y / 2);
  execute!(stdout(), crossterm::cursor::MoveTo(min_x + x, min_y + y))?;
  Ok(())
}

/// Assert that a position (x, y) is within the bounds on state without
/// checking that the bounds have already been set.
pub fn assert_is_within_bounds_unchecked(x: u16, y: u16, state: &CirnoState) -> Result<(), CirnoError> {
  let bound_x = state.meta.bounds.x;
  let bound_y = state.meta.bounds.y;
  if x > bound_x - 1 || y > bound_y - 1 { // zero
    return Err(CirnoError::OutOfBounds)
  }
  Ok(())
}

pub fn read_line() -> Result<String, io::Error> {
  let mut line = String::new();
  while let Ok(Event::Key(KeyEvent { code, modifiers: _, kind, state: _ })) = crossterm::event::read() {
    if !matches!(kind, KeyEventKind::Press) {
      continue;
    }
    match code {
      KeyCode::Enter => { break; },
      KeyCode::Backspace => {
        if line.eq("") {
          break;
        }
        line.pop();
        backspace()?;
      },
      KeyCode::Char(c) => {
        line.push(c);
        execute!(stdout(), crossterm::style::Print(c))?;
      },
      _ => {},
    }
  }
  Ok(line)
}

pub fn read_key_presses(n: usize) -> Result<Option<String>, io::Error> {
  let mut sequence = String::new();
  while let Ok(Event::Key(KeyEvent { code, modifiers: _, kind, state: _ })) = crossterm::event::read() {
    if !matches!(kind, KeyEventKind::Press) {
      continue;
    }
    match code {
      KeyCode::Esc => return Ok(None),
      KeyCode::Char(c) => { sequence.push(c); },
      _ => {},
    }
    if sequence.len() == n {
      break;
    }
  }
  Ok(Some(sequence))
}
