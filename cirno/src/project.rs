use crate::{error::CirnoError, terminal::{assert_is_within_bounds_unchecked, move_within_bounds, EventResult}, CirnoState};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::stdout;
use crossterm::execute;

#[derive(Clone, Debug, Default)]
pub struct Vector2 {
  pub x: u16,
  pub y: u16,
}

#[derive(Clone, Debug, Default)]
pub struct Region {
  pub position: Vector2,
  pub size: Vector2,
}

impl Region {
  /// Return whether a region is overlapping another.
  pub fn overlapping(&self, other_region: Region) -> bool {
    // region bounds
    let (r1_min_x, r1_min_y) = (self.position.x, self.position.y);
    let (r1_max_x, r1_max_y) = (r1_min_x + self.size.x - 1, r1_min_y + self.size.y - 1);
    let (r2_min_x, r2_min_y) = (other_region.position.x, other_region.position.y);
    let (r2_max_x, r2_max_y) = (r2_min_x + other_region.size.x - 1, r2_min_y + other_region.size.y - 1);
    // valid locations
    let r2_min_x_inside = r2_min_x >= r1_min_x && r2_min_x <= r1_max_x;
    let r2_max_x_inside = r2_max_x >= r1_min_x && r2_max_x <= r1_max_x;
    let r2_min_y_inside = r2_min_y >= r1_min_y && r2_min_y <= r1_max_y;
    let r2_max_y_inside = r2_max_y >= r1_min_y && r2_max_y <= r1_max_y;
    let r2_top_left_inside = r2_min_x_inside && r2_min_y_inside;
    let r2_top_right_inside = r2_max_x_inside && r2_min_y_inside;
    let r2_bottom_left_inside = r2_min_x_inside && r2_max_y_inside;
    let r2_bottom_right_inside = r2_max_x_inside && r2_max_y_inside;
    r2_top_left_inside || r2_top_right_inside || r2_bottom_left_inside || r2_bottom_right_inside
  }
}

#[derive(Clone, Debug, Default)]
// a value that a pin can have
pub enum Value {
  And(Vec<String>),
  Gnd,
  #[default]
  None,
  Vcc,
}

#[derive(Clone, Debug)]
// an attribute that an object can have
pub enum Attribute {
  Bounds(Vector2),
  Label(String),
  Num(u16),
  Position(Vector2),
  Type(String),
  Value(Value),
  YCoordinate(u16),
}

impl std::fmt::Display for Attribute {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{:?}", self)
  }
}

#[derive(Clone, Debug, Default)]
pub struct Chip {
  pub t: String,
  pub position: Vector2,
}

impl Chip {
  pub fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Type(t) => self.t = t,
      Attribute::Position(vec2) => self.position = vec2,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "chip".to_string())),
    }
    Ok(())
  }
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    if self.t.eq("") {
      return Err(CirnoError::MissingAttribute("chip type".to_string()))
    }
    // TODO: chip position
    Ok(())
  }
  pub fn region(&self, state: &CirnoState) -> Option<Region> {
    let pins = state.cic_data.get(&self.t).unwrap().to_owned();
    let width = u16::try_from(pins.len() / 2).unwrap();
    let position = Vector2 { x: self.position.x, y: self.position.y };
    let size = Vector2 { x: width, y: 3 };
    Some(Region { position, size })
  }
  pub fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let x = self.position.x;
    let y = self.position.y;
    let pins = state.cic_data.get(&self.t).unwrap().to_owned();
    let width = pins.len() / 2;
    // bounds check
    for pin in pins {
      let position = match pin {
        Object::Pin(pin) => pin.position,
        _ => todo!(),
      };
      assert_is_within_bounds_unchecked(x + position.x, y + position.y, state)?;
    }
    // rendering
    move_within_bounds(x, y, state)?;
    execute!(stdout(), crossterm::style::Print(".".repeat(width.into())))?;
    move_within_bounds(x, y + 2, state)?;
    execute!(stdout(), crossterm::style::Print(".".repeat(width.into())))?;
    Ok(())
  }
}

#[derive(Clone, Debug, Default)]
pub struct Meta {
  pub bounds: Vector2,
}

impl Meta {
  pub fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Bounds(vec2) => self.bounds = vec2,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "meta".to_string())),
    }
    Ok(())
  }
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    if self.bounds.x == 0 && self.bounds.y == 0 {
      return Err(CirnoError::MissingAttribute("bounds".to_string()))
    }
    if self.bounds.x == 0 || self.bounds.y == 0 {
      return Err(CirnoError::NamelessInvalidValueForAttribute("bounds".to_string()))
    }
    Ok(())
  }
  pub fn region(&self, _state: &CirnoState) -> Option<Region> {
    None
  }
  pub fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let bound_x = state.meta.bounds.x;
    let bound_y = state.meta.bounds.y;
    let center_x = state.columns / 2;
    let center_y = state.rows / 2;
    let min_x = center_x - (bound_x / 2) - 1;
    let min_y = center_y - (bound_y / 2) - 1;
    let max_x = center_x + (bound_x / 2);
    let max_y = center_y + (bound_y / 2);
    execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey))?;
    // top border
    execute!(stdout(), crossterm::cursor::MoveTo(min_x, min_y))?;
    execute!(stdout(), crossterm::style::Print("~".repeat((bound_x + 2).into())))?;
    // side borders
    let mut i = 1;
    while i < bound_y + 1 {
      // execute!(stdout(), crossterm::style::Print(format!("{}{}{}", "~", " ".repeat(state.bound_x.into()), "~")))?;
      // left border
      execute!(stdout(), crossterm::cursor::MoveTo(min_x, min_y + i))?;
      execute!(stdout(), crossterm::style::Print("~"))?;
      // right border
      execute!(stdout(), crossterm::cursor::MoveTo(max_x, min_y + i))?;
      execute!(stdout(), crossterm::style::Print("~"))?;
      i = i + 1;
    }
    // bottom border
    execute!(stdout(), crossterm::cursor::MoveTo(min_x, max_y))?;
    execute!(stdout(), crossterm::style::Print("~".repeat((bound_x + 2).into())))?;
    execute!(stdout(), crossterm::style::ResetColor)?;
    Ok(())
  }
}

#[derive(Clone, Debug, Default)]
pub struct Net {
  pub t: String,
  pub y: u16,
}

impl Net {
  pub fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Type(t) => self.t = t,
      Attribute::YCoordinate(y) => self.y = y,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "net".to_string())),
    }
    Ok(())
  }
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    match self.t.as_str() {
      "vcc" | "gnd" => {},
      "" => return Err(CirnoError::MissingAttribute("net type".to_string())),
      t => return Err(CirnoError::InvalidValueForAttribute(t.to_string(), "net type".to_string())),
    };
    // TODO: net y
    Ok(())
  }
  pub fn region(&self, state: &CirnoState) -> Option<Region> {
    let position = Vector2 { x: 0, y: self.y };
    let size = Vector2 { x: state.meta.bounds.x, y: 1 };
    Some(Region { position, size })
  }
  pub fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let y = self.y;
    let bound_x = state.meta.bounds.x;
    // bounds check
    assert_is_within_bounds_unchecked(0, y, state)?;
    // rendering
    move_within_bounds(0, y, state)?;
    match self.t.as_str() {
      "vcc" => {
        execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Red))?;
        execute!(stdout(), crossterm::style::Print("+".repeat(bound_x.into())))?;
      },
      "gnd" => {
        execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Blue))?;
        execute!(stdout(), crossterm::style::Print("-".repeat(bound_x.into())))?;
      },
      _t => unreachable!(),
    }
    execute!(stdout(), crossterm::style::ResetColor)?;
    Ok(())
  }
}

// impl Default for Net {
//   fn default() -> Net {
//     Net { t: String::new(), y: -1 }
//   }
// }

#[derive(Clone, Debug, Default)]
pub struct Pin {
  pub label: String,
  pub num: u16,
  pub position: Vector2,
  pub value: Value,
}

impl Pin {
  pub fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Label(label) => self.label = label,
      Attribute::Num(num) => self.num = num,
      Attribute::Position(vec2) => self.position = vec2,
      Attribute::Value(value) => self.value = value,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "pin".to_string())),
    }
    Ok(())
  }
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    // TODO: pin num, pin position
    Ok(())
  }
  pub fn region(&self, _state: &CirnoState) -> Option<Region> {
    let position = Vector2 { x: self.position.x, y: self.position.y };
    let size = Vector2 { x: 1, y: 1 };
    Some(Region { position, size })
  }
  // TODO: dead code?
  pub fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    // this call returns the position of the top left corner of the parent chip
    let (col, row) = crossterm::cursor::position()?;
    let x = col + self.position.x;
    let y = row + self.position.y;
    // bounds check
    assert_is_within_bounds_unchecked(x, y, state)?;
    // rendering
    move_within_bounds(x, y, state)?;
    execute!(stdout(), crossterm::style::Print("."))?;
    Ok(())
  }
}

// an object that cirno can render
#[derive(Clone)]
pub enum Object {
  Chip(Chip),
  Meta(Meta),
  Net(Net),
  Pin(Pin),
}

impl Object {
  pub fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match self {
      Object::Chip(chip) => chip.apply_attribute(attribute),
      Object::Meta(meta) => meta.apply_attribute(attribute),
      Object::Net(net) => net.apply_attribute(attribute),
      Object::Pin(pin) => pin.apply_attribute(attribute),
    }
  }
  pub fn verify(&mut self) -> Result<(), CirnoError> {
    match self {
      Object::Chip(chip) => chip.verify(),
      Object::Meta(meta) => meta.verify(),
      Object::Net(net) => net.verify(),
      Object::Pin(pin) => pin.verify(),
    }
  }
  pub fn region(&self, state: &CirnoState) -> Option<Region> {
    match self {
      Object::Chip(chip) => chip.region(state),
      Object::Meta(meta) => meta.region(state),
      Object::Net(net) => net.region(state),
      Object::Pin(pin) => pin.region(state),
    }
  }
  pub fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    match self {
      Object::Chip(chip) => chip.render(state),
      Object::Meta(meta) => meta.render(state),
      Object::Net(net) => net.render(state),
      Object::Pin(pin) => pin.render(state),
    }
  }
}

impl Debug for Object {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str("Object::")?;
    match self {
      Object::Chip(chip) => chip.fmt(f),
      Object::Meta(meta) => meta.fmt(f),
      Object::Net(net) => net.fmt(f),
      Object::Pin(pin) => pin.fmt(f),
    }
  }
}

#[derive(Clone)]
pub struct Mode {
  pub mode_set_cb: fn(&mut CirnoState) -> Result<(), anyhow::Error>,
  pub key_event_cb: fn(crossterm::event::KeyEvent, &mut CirnoState) -> Result<EventResult, anyhow::Error>,
  pub resize_event_cb: fn(&mut CirnoState) -> Result<EventResult, anyhow::Error>,
  pub key_commands: HashMap<char, fn(&mut CirnoState) -> Result<EventResult, anyhow::Error>>,
  // TODO: state needed or not?
  pub commands: HashMap<String, fn(&mut CirnoState) -> Result<EventResult, anyhow::Error>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Modes {
  Console,
  Normal,
}
