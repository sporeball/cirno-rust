use crate::{project::{Mode, Modes, Object}, terminal::KeyEventResult};
use std::io::{self};
use std::io::stdout;
use crossterm::{execute, event::Event, terminal::Clear};

pub mod logger;
pub mod modes;
pub mod parser;
pub mod project;
pub mod terminal;

pub struct CirnoState {
  pub mode: Modes,
  pub cursor_x: u16,
  pub cursor_y: u16,
  pub objects: Vec<Object>,
}

impl CirnoState {
  pub fn get_mode(&mut self) -> Mode {
    match self.mode {
      Modes::Normal => crate::modes::normal::get(),
    }
  }
  pub fn event_loop(&mut self) -> Result<(), io::Error> {
    loop {
      match crossterm::event::read()? {
        // key event
        Event::Key(event) => {
          let res: KeyEventResult = (self.get_mode().key_event_cb)(event, self);
          if matches!(res, KeyEventResult::Exit) {
            return Ok(())
          }
        },
        _ => (),
      }
    }
  }
  pub fn render(&mut self) -> Result<(), io::Error> {
    execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    for object in self.objects.clone() {
      object.render(self);
    }
    Ok(())
  }
}
