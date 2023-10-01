use crate::{error::CirnoError, project::{Mode, Modes, Object}, terminal::KeyEventResult};
use std::io::{self};
use std::io::stdout;
use crossterm::{execute, event::Event};
use thiserror::Error;

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
  pub bound_x: u16,
  pub bound_y: u16,
  pub cursor_x: u16,
  pub cursor_y: u16,
  pub objects: Vec<Object>,
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
    execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
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
    let meta_unwrapped = meta.unwrap();
    // apply attributes to self
    self.bound_x = meta_unwrapped.bounds.x;
    self.bound_y = meta_unwrapped.bounds.y;
    // crate::logger::debug(&self);
    Ok(())
  }
}
