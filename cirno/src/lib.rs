use crate::{error::{CirnoError, try_to}, project::{Chip, Meta, Mode, Modes, Object, ObjectEnum, Value, Vector2, Voltage}, terminal::EventResult};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use crossterm::{event::Event, style::Color};
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
  pub objects: Rc<RefCell<Vec<ObjectEnum>>>,
  pub meta: Meta,
  pub error: String,
  pub cic_data: HashMap<String, Vec<ObjectEnum>>,
  pub repeat_amount: u16,
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
  /// The top left pin of each chip added will have a position of (0, 0).
  pub fn set_cic_data(&mut self) -> Result<(), anyhow::Error> {
    let binding = self.objects.borrow();
    let mut types: Vec<&str> = binding
      .iter()
      .filter_map(|x| match x {
        ObjectEnum::Chip(Chip { t, region: _ }) => Some(t.as_str()),
        _ => None,
      })
      .collect();
    types.sort();
    types.dedup();
    for t in types {
      let contents = stdlib(t)?;
      let mut v = parse(&contents)?;
      let len = v.len() / 2;
      for (index, pin) in v.iter_mut().enumerate() {
        let ObjectEnum::Pin(pin) = pin else { unreachable!(); };
        pin.set_temp_region_position(index, len)?;
        pin.set_region_size(self)?;
      }
      self.cic_data.insert(t.to_string(), v);
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
  /// Set the `label` property of every wire object.
  pub fn set_wire_labels(&mut self) -> Result<(), anyhow::Error> {
    let mut counts: HashMap<Color, u32> = HashMap::from([
      (Color::Red, 0),
      (Color::Green, 0),
      (Color::Yellow, 0),
      (Color::Blue, 0),
      (Color::Magenta, 0),
      (Color::Cyan, 0),
    ]);
    let mut binding = self.objects.borrow_mut();
    let wires = binding
      .iter_mut()
      .filter_map(|x| match x {
        ObjectEnum::Wire(wire) => Some(wire),
        _ => None,
      });
    for wire in wires {
      let count: u32 = *counts.get(&wire.color).unwrap();
      if count > 51 {
        return Err(CirnoError::TooManyWiresOfColor(color_to_string(wire.color)).into())
      } else if count > 25 {
        wire.label = char::from_u32(39 + count).unwrap(); // A..Z
      } else {
        wire.label = char::from_u32(97 + count).unwrap(); // a..z
      }
      if let Some(c) = counts.get_mut(&wire.color) {
        *c += 1;
      }
    }
    Ok(())
  }
  /// Set the `voltage` property of every pin object.
  pub fn set_pin_voltages(&mut self) -> Result<(), anyhow::Error> {
    let mut binding = self.objects.borrow_mut();
    let (mut pins, mut wires, mut nets) = binding.iter_mut().fold((vec![], vec![], vec![]), |mut acc, x| {
      match x {
        ObjectEnum::Pin(pin) => acc.0.push(pin),
        ObjectEnum::Wire(wire) => acc.1.push(wire),
        ObjectEnum::Net(net) => acc.2.push(net),
        _ => {},
      };
      acc
    });
    for pin in pins.iter_mut() {
      let Some(wire) = wires.iter_mut().find(|x| x.is_connected_to(pin.region.position)) else { continue; };
      let Some(net) = nets.iter_mut().find(|x| x.region.overlapping_vec2(wire.from) || x.region.overlapping_vec2(wire.to)) else { continue; };
      pin.voltage = match net.t.as_str() {
        "vcc" => Voltage::High,
        "gnd" => Voltage::Low,
        _ => unreachable!(),
      };
    }
    Ok(())
  }
  /// Replace the chips in `objects` with the corresponding pins from `cic_data`, updating the
  /// position of each.
  pub fn convert_chips(&mut self) -> Result<(), anyhow::Error> {
    let mut v: Vec<ObjectEnum> = vec![];
    let mut chip_counts: HashMap<String, u32> = HashMap::new();
    let binding = self.objects.borrow();
    for object in binding.iter().cloned() { // objects
      if let ObjectEnum::Chip(chip) = object {
        let short_chip_type = short_chip_type(chip.t.clone());
        // update chip_counts based on chip type
        if !chip_counts.contains_key(&chip.t) {
          chip_counts.insert(chip.t.clone(), 0);
        }
        if let Some(c) = chip_counts.get_mut(&chip.t) {
          *c += 1;
        }
        let c = chip_counts.get(&chip.t).unwrap();
        // for each pin
        for pin in self.cic_data.get(&chip.t).unwrap().iter().cloned() {
          let ObjectEnum::Pin(mut pin) = pin else { unreachable!(); };
          // update position
          pin.region.position.x += chip.region.position.x;
          pin.region.position.y += chip.region.position.y;
          // update label
          if pin.label != "" {
            pin.label = format!("{}_{}_{}", pin.label, short_chip_type, c);
          }
          // update value
          match pin.value {
            Value::And(labels) => {
              let mut lv: Vec<String> = vec![];
              for label in labels {
                lv.push(format!("{}_{}_{}", label, short_chip_type, c));
              }
              pin.value = Value::And(lv);
            },
            _ => {},
          }
          // push the updated pin
          v.push(ObjectEnum::Pin(pin));
        }
      } else {
        v.push(object);
      }
    }
    drop(binding); // avoids a panic
    self.objects.replace(v);
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
          } else if matches!(res, EventResult::Ok) {
            self.repeat_amount = 0;
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
  /// Return the meta object, or CirnoError::MetaObjectError if it cannot be found.
  pub fn find_meta(&mut self) -> Result<Meta, CirnoError> {
    self.objects
      .borrow()
      .iter()
      .find_map(|x| match x {
        ObjectEnum::Meta(meta) => Some(meta.to_owned()),
        _ => None,
      })
      .ok_or(CirnoError::MetaObjectError)
  }
  /// Verify the current state.
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    // state.meta.bounds should be set
    let bound_x = self.meta.bounds.x;
    let bound_y = self.meta.bounds.y;
    if bound_x == 0 && bound_y == 0 {
      return Err(CirnoError::MetaObjectError)
    }
    // the terminal should be large enough to render the entire bounds
    // 2 extra columns and rows are added to account for the border
    if bound_x + 2 > self.columns || bound_y + 2 > self.rows {
      return Err(CirnoError::TerminalTooSmall)
    }
    Ok(())
  }
  /// Verify that no objects overlap with each other.
  pub fn verify_overlap(&mut self) -> Result<(), CirnoError> {
    for (index, object) in self.objects.borrow().iter().enumerate() {
      let Some(region) = object.get_region() else { continue };
      for (other_index, other_object) in self.objects.borrow().iter().enumerate().filter(|x| x.0 > index) {
        let Some(other_region) = other_object.get_region() else { continue };
        if region.overlapping(other_region) {
          return Err(CirnoError::OverlappingRegion(index, other_index))
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
      let contents = fs::read_to_string(path)?;
      Ok(contents)
    },
    x => Err(CirnoError::InvalidFiletype(x.to_string()).into()),
  }
}

pub fn stdlib(filename: &str) -> Result<String, anyhow::Error> {
  let out_dir = std::env::var_os("OUT_DIR").unwrap();
  let path = Path::new(&out_dir).join(format!("stdlib/{}.cic", filename));
  if !path.exists() {
    return Err(CirnoError::NotFoundInStdlib(filename.to_string()).into())
  }
  let contents = fs::read_to_string(path)?;
  Ok(contents)
}

pub fn color_to_string(color: Color) -> String {
  match color {
    Color::Red => "red".to_string(),
    Color::Green => "green".to_string(),
    Color::Yellow => "yellow".to_string(),
    Color::Blue => "blue".to_string(),
    Color::Magenta => "magenta".to_string(),
    Color::Cyan => "cyan".to_string(),
    _ => todo!(),
  }
}

pub fn short_chip_type(t: String) -> String {
  t.split('/').collect::<Vec<&str>>().last().unwrap().to_string()
}
