use crate::{project::{Mode, Modes, Object}, terminal::KeyEventResult};
use std::io::{self};
use std::io::stdout;
use crossterm::{execute, event::Event, terminal::Clear};

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
  pub bound_x: i32,
  pub bound_y: i32,
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
  pub fn apply_meta(&mut self) -> () {
    // get meta object
    let meta = self.objects
      .clone()
      .into_iter()
      .find_map(|x| match x {
        Object::Meta(meta) => Some(meta),
          _ => None,
        })
      .unwrap();
    // apply attributes to self
    self.bound_x = meta.bounds.x;
    self.bound_y = meta.bounds.y;
    // crate::logger::debug(&self);
  }
}
