use std::any::Any;
use strum_macros::Display;

#[derive(Clone, Debug)]
// a value that a pin can have
pub enum Value {
  And(Vec<String>),
  Gnd,
  Vcc,
}

#[derive(Debug)]
pub struct Label {
  pub value: String,
}

#[derive(Debug)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug)]
pub struct Type {
  pub t: String,
}

#[derive(Debug)]
pub struct YCoordinate {
  pub y: i32,
}

#[derive(Debug)]
pub struct Chip {
  t: String,
  position: Position,
}

#[derive(Clone, Display, Debug)]
// an attribute that an object can have
pub enum Attribute {
  Label(String),
  Position(i32, i32),
  Type(String),
  Value(Value),
  YCoordinate(i32),
}

// #[derive(Debug)]
// pub enum ObjectType {
//   Chip,
//   Net,
//   Pin,
// }

#[derive(Debug)]
// an object that cirno can render
// pub struct Object {
//   pub t: ObjectType,
//   pub attributes: Vec<Attribute>,
// }

// an object that cirno can render
pub enum Object {
  Chip { t: Option<String>, position: Option<Position> },
  Net { t: Option<String>, y: Option<i32> },
  // Pin(...)
}