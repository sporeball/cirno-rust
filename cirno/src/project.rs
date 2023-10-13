use crate::{CirnoState, error::CirnoError, parser::parse, terminal::{KeyEventResult, assert_is_within_bounds_unchecked, move_within_bounds}};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::stdout;
use crossterm::execute;

#[derive(Clone, Debug, Default)]
pub struct Vector2 {
  pub x: u16,
  pub y: u16,
}

#[derive(Clone, Debug)]
// a value that a pin can have
pub enum Value {
  And(Vec<String>),
  Gnd,
  None,
  Vcc,
}

impl Default for Value {
  fn default() -> Value {
    Value::None
  }
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
      a => return Err(CirnoError::InvalidChipAttribute(a.to_string())),
    }
    Ok(())
  }
  pub fn render(self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let x = self.position.x;
    let y = self.position.y;
    // read .cic based on type field
    let filename = format!("../stdlib/{}{}", self.t, ".cic"); // TODO: make this stronger, make sure this doesn't break
    let cic: ParseResult = parse(&filename)?;
    let pins = match cic {
      ParseResult::Cic(Cic { pins }) => pins,
      ParseResult::Cip(_) => unreachable!(),
    };
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
      a => return Err(CirnoError::InvalidMetaAttribute(a.to_string())),
    }
    Ok(())
  }
  pub fn render(self, state: &CirnoState) -> Result<(), anyhow::Error> {
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
      a => return Err(CirnoError::InvalidNetAttribute(a.to_string())),
    }
    Ok(())
  }
  pub fn render(self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let y = self.y;
    let bound_x = state.meta.bounds.x;
    // bounds check
    assert_is_within_bounds_unchecked(0, y, state)?;
    // rendering
    move_within_bounds(0, y, state)?;
    if self.t.eq("vcc") {
      execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Red))?;
      execute!(stdout(), crossterm::style::Print("+".repeat(bound_x.into())))?;
    } else {
      execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Blue))?;
      execute!(stdout(), crossterm::style::Print("-".repeat(bound_x.into())))?;
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
      a => return Err(CirnoError::InvalidPinAttribute(a.to_string())),
    }
    Ok(())
  }
  // TODO: dead code?
  pub fn render(self, state: &CirnoState) -> Result<(), anyhow::Error> {
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
  pub fn render(self, state: &CirnoState) -> Result<(), anyhow::Error> {
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

#[derive(Debug, Default)]
pub struct Cic {
  pub pins: Vec<Object>,
}

#[derive(Debug, Default)]
pub struct Cip {
  pub objects: Vec<Object>,
}

// the result of parsing a .cic or .cip file
pub enum ParseResult {
  Cic(Cic),
  Cip(Cip),
  // Invalid(String),
}

impl ParseResult {
  pub fn apply(&mut self, ast: Vec<Object>) {
    match self {
      ParseResult::Cic(cic) => cic.pins = ast,
      ParseResult::Cip(cip) => cip.objects = ast,
    }
  }
}

impl Debug for ParseResult {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    f.write_str("ParseResult::")?;
    match self {
      ParseResult::Cic(cic) => cic.fmt(f),
      ParseResult::Cip(cip) => cip.fmt(f),
    }
  }
}

#[derive(Clone)]
pub struct Mode {
  pub key_event_cb: fn(crossterm::event::KeyEvent, &mut CirnoState) -> Result<KeyEventResult, anyhow::Error>,
  pub key_commands: HashMap<char, fn(&mut CirnoState) -> Result<KeyEventResult, anyhow::Error>>,
  // TODO: new return type (not KeyEventResult)
  // TODO: state needed or not?
  pub commands: HashMap<String, fn(&mut CirnoState) -> Result<KeyEventResult, anyhow::Error>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Modes {
  Normal,
}
