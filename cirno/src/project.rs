use std::any::Any;
use std::fmt::Debug;
use strum_macros::Display;

#[derive(Debug)]
pub struct Label {
  pub value: String,
}

#[derive(Debug, Default)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Default)]
pub struct Type {
  pub t: String,
}

#[derive(Debug)]
// a value that a pin can have
pub enum Value {
  And(Vec<String>),
  Gnd,
  Vcc,
}

#[derive(Debug, Default)]
pub struct YCoordinate {
  pub y: i32,
}

#[derive(Display, Debug)]
// an attribute that an object can have
pub enum Attribute {
  Label(String),
  Position(Position),
  Type(String),
  Value(Value),
  YCoordinate(i32),
}

#[derive(Debug, Default)]
pub struct Chip {
  pub t: String,
  pub position: Position,
}

impl Chip {
  pub fn apply_attribute(&mut self, attribute: Attribute) {
    match attribute {
      Attribute::Type(t) => self.t = t,
      Attribute::Position(position) => self.position = position,
      a => panic!("could not apply {} to chip", a),
    }
  }
}

#[derive(Debug)]
pub struct Net {
  pub t: String,
  pub y: i32,
}

impl Net {
  pub fn apply_attribute(&mut self, attribute: Attribute) {
    match attribute {
      Attribute::Type(t) => self.t = t,
      Attribute::YCoordinate(y) => self.y = y,
      a => panic!("could not apply {} to net", a),
    }
  }
}

impl Default for Net {
  fn default() -> Net {
    Net { t: String::new(), y: -1 }
  }
}

// an object that cirno can render
pub enum Object {
  Chip(Chip),
  Net(Net),
  // Pin(...)
}

impl Object {
  pub fn apply_attribute(&mut self, attribute: Attribute) {
    match self {
      Object::Chip(chip) => chip.apply_attribute(attribute),
      Object::Net(net) => net.apply_attribute(attribute),
    }
  }
}

impl Debug for Object {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    f.write_str("Object::")?;
    match self {
      Object::Chip(chip) => chip.fmt(f),
      Object::Net(net) => net.fmt(f),
    }
  }
}

#[derive(Debug, Default)]
pub struct Cic {
}

#[derive(Debug, Default)]
pub struct Cip {
  objects: Vec<Object>,
}

// the result of parsing a .cic or .cip file
pub enum ParseResult {
  Cic(Cic),
  Cip(Cip),
}

impl ParseResult {
  pub fn apply(&mut self, ast: Vec<Object>) {
    match self {
      ParseResult::Cic(cic) => todo!(),
      ParseResult::Cip(cip) => cip.objects = ast,
    }
  }
  pub fn verify(&mut self) {
    match self {
      ParseResult::Cic(cic) => todo!(),
      ParseResult::Cip(cip) => todo!(),
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