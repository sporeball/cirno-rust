use crate::{error::CirnoError, project::{Meta, Mode, Modes, Object, Vector2}, terminal::KeyEventResult};
use std::io::{self};
use crossterm::event::Event;

pub mod bar;
pub mod error;
pub mod logger;
pub mod modes;
pub mod parser;
pub mod project;
pub mod terminal;

#[derive(Debug)]
pub struct CirnoState {
  pub columns: u16,
  pub rows: u16,
  pub mode: Modes,
  pub cursor: Vector2,
  pub objects: Vec<Object>,
  pub meta: Meta,
  pub errors: Vec<String>,
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
          let res = (self.get_mode().key_event_cb)(event, self).unwrap();
          if matches!(res, KeyEventResult::Exit) {
            return Ok(())
          }
        },
        _ => (),
      }
    }
  }
  pub fn render(&mut self) -> Result<(), anyhow::Error> {
    for object in self.objects.clone() {
      object.render(self)?;
    }
    Ok(())
  }
  pub fn apply_meta(&mut self) -> Result<(), CirnoError> {
    // get meta object
    let meta = self.objects
      .clone()
      .into_iter()
      .find_map(|x| match x {
        Object::Meta(meta) => Some(meta),
        _ => None,
      });
    if meta.is_none() {
      return Err(CirnoError::MissingMetaObject)
    }
    self.meta = meta.unwrap();
    // crate::logger::debug(&self);
    Ok(())
  }
  /// Verify the current state.
  /// This function should be called only after the `meta` property has been
  /// successfully set.
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    let bound_x = self.meta.bounds.x;
    let bound_y = self.meta.bounds.y;
    // the terminal should be large enough to render the entire bounds
    if bound_x > self.columns || bound_y > self.rows {
      return Err(CirnoError::TerminalTooSmall)
    }
    Ok(())
  }
}
