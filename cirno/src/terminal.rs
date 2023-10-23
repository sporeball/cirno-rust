use crate::{CirnoState, error::CirnoError};
use std::io;
use std::io::stdout;
use crossterm::{event::KeyEvent, execute, terminal::ClearType};

pub enum KeyEventResult {
  Ok,
  Err,
  Exit,
}

pub fn clear_all() -> Result<(), io::Error> {
  execute!(stdout(), crossterm::terminal::Clear(ClearType::All))?;
  Ok(())
}

pub fn enter() -> Result<(), io::Error> {
  crossterm::terminal::enable_raw_mode()?;
  execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
  execute!(stdout(), crossterm::terminal::DisableLineWrap)?;
  execute!(stdout(), crossterm::cursor::Hide)?;
  Ok(())
}

pub fn exit() -> Result<(), io::Error> {
  execute!(stdout(), crossterm::cursor::Show)?;
  execute!(stdout(), crossterm::terminal::EnableLineWrap)?;
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
  while let Ok(crossterm::event::Event::Key(KeyEvent { code, .. })) = crossterm::event::read() {
    match code {
      crossterm::event::KeyCode::Enter => { break; },
      crossterm::event::KeyCode::Backspace => {
        if line.eq("") {
          break;
        }
        line.pop();
        backspace()?;
      },
      crossterm::event::KeyCode::Char(c) => {
        line.push(c);
        execute!(stdout(), crossterm::style::Print(c))?;
      },
      _ => {},
    }
  }
  Ok(line)
}
