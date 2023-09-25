use crate::{project::{Mode, Modes}, terminal::KeyEventResult};
use std::io::{self};
use crossterm::event::Event;

pub mod logger;
pub mod modes;
pub mod parser;
pub mod project;
pub mod terminal;

pub struct CirnoState {
  pub mode: Modes,
  pub cursor_x: i32,
  pub cursor_y: i32,
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
}