use crate::{error::{CirnoError, try_to}, project::{Meta, Mode, Modes, Object, Vector2}, terminal::EventResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use crossterm::event::Event;
use parser::parse;

pub mod bar;
pub mod cursor;
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
  pub objects: Rc<RefCell<Vec<Object>>>,
  pub meta: Meta,
  pub error: String,
  pub cic_data: HashMap<String, Vec<Object>>,
}

impl CirnoState {
  /// Get the current mode.
  pub fn get_mode(&mut self) -> Mode {
    match self.mode {
      Modes::Console => crate::modes::console::get(),
      Modes::Normal => crate::modes::normal::get(),
    }
  }
  /// Set the current mode.
  pub fn set_mode(&mut self, mode: Modes) -> Result<(), anyhow::Error> {
    self.mode = mode;
    try_to((self.get_mode().mode_set_cb)(self), self)?;
    Ok(())
  }
  /// Populate `cic_data` based on the chip types in `objects`.
  pub fn set_cic_data(&mut self) -> Result<(), anyhow::Error> {
    let mut types: Vec<String> = self.objects
      .borrow()
      .iter()
      .filter_map(|x| match x {
        Object::Chip(chip) => Some(chip.to_owned().t),
        _ => None,
      })
      .collect();
    types.sort();
    types.dedup();
    for t in types {
      if !self.cic_data.contains_key(&t) {
        let contents = stdlib(&t)?;
        let v = parse(&contents)?;
        self.cic_data.insert(t, v);
      }
    }
    // logger::debug(format!("{:?}", self.cic_data));
    Ok(())
  }
  /// Set the `region.size` property of every object.
  pub fn set_region_sizes(&mut self) -> Result<(), anyhow::Error> {
    for object in self.objects.borrow_mut().iter_mut() {
      object.set_region_size(self)?;
    }
    Ok(())
  }
  /// cirno's event loop.
  /// This function blocks until cirno is explicitly quit.
  pub fn event_loop(&mut self) -> Result<(), anyhow::Error> {
    loop {
      match crossterm::event::read()? {
        Event::Key(event) => {
          let res = (self.get_mode().key_event_cb)(event, self).unwrap();
          if matches!(res, EventResult::Exit) {
            return Ok(())
          }
        },
        Event::Resize(columns, rows) => {
          self.columns = columns;
          self.rows = rows;
          try_to((self.get_mode().resize_event_cb)(self), self)?;
        },
        _ => (),
      }
    }
  }
  /// Render all objects.
  pub fn render(&mut self) -> Result<(), anyhow::Error> {
    for object in self.objects.borrow().iter() {
      object.render(self)?;
    }
    cursor::render(self)?;
    cursor::report(self)?;
    Ok(())
  }
  /// Return the meta object, or CirnoError::MissingMetaObject if it cannot be found.
  pub fn find_meta(&mut self) -> Result<Meta, CirnoError> {
    self.objects
      .borrow()
      .iter()
      .find_map(|x| match x {
        Object::Meta(meta) => Some(meta.to_owned()),
        _ => None,
      })
      .ok_or(CirnoError::MissingMetaObject)
  }
  /// Verify the current state.
  /// This function should be called only after the `meta` property has been
  /// successfully set.
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    let bound_x = self.meta.bounds.x;
    let bound_y = self.meta.bounds.y;
    // the terminal should be large enough to render the entire bounds
    // 2 extra columns and rows are added to account for the border
    if bound_x + 2 > self.columns || bound_y + 2 > self.rows {
      return Err(CirnoError::TerminalTooSmall)
    }
    // no regions should overlap with each other
    for (index, object) in self.objects.borrow().iter().enumerate() {
      let option_region = match object.to_owned() {
        Object::Chip(chip) => Some(chip.region),
        Object::Meta(_meta) => None,
        Object::Net(net) => Some(net.region),
        Object::Pin(pin) => Some(pin.region),
      };
      for (other_index, other_object) in self.objects.borrow().iter().enumerate().filter(|x| x.0 > index) {
        let option_other_region = match other_object.to_owned() {
          Object::Chip(chip) => Some(chip.region),
          Object::Meta(_meta) => None,
          Object::Net(net) => Some(net.region),
          Object::Pin(pin) => Some(pin.region),
        };
        if option_region.is_some() && option_other_region.is_some() {
          let region = option_region.clone().unwrap();
          let other_region = option_other_region.unwrap();
          if region.overlapping(other_region) {
            return Err(CirnoError::OverlappingRegion(index, other_index))
          }
        }
      }
    }
    Ok(())
  }
}

/// Read a file and return its contents.
/// Only .cic and .cip files are accepted.
pub fn read(filename: &str) -> Result<String, anyhow::Error> {
  let path = Path::new(filename);
  let extension = path.extension();
  if extension.is_none() {
    return Err(CirnoError::InvalidFile(filename.to_string()).into());
  }
  match extension.unwrap().to_str().unwrap() { // converts from Option<&OsStr> to &str
    "cic" | "cip" => {
      let contents = std::fs::read_to_string(path)?;
      Ok(contents)
    },
    x => Err(CirnoError::InvalidFiletype(x.to_string()).into()),
  }
}

pub fn stdlib(filename: &str) -> Result<String, anyhow::Error> {
  let out_dir = std::env::var_os("OUT_DIR").unwrap();
  let path = Path::new(&out_dir)
    .join(format!("stdlib/{}.cic", filename));
  if !path.exists() {
    return Err(CirnoError::NotFoundInStdlib(filename.to_string()).into())
  }
  let contents: String = fs::read_to_string(path)?;
  Ok(contents)
}