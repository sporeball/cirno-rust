enum NetType {
  Vcc,
  Gnd,
}

// types
#[derive(Debug)]
pub struct YCoordinate(pub i32);
#[derive(Debug)]
pub struct Position(pub i32, pub i32);

#[derive(Debug)]
pub enum Attribute {
  YCoordinate(i32),
  Position(i32, i32),
}

#[derive(Debug)]
pub enum Object {
  Chip(String, Position),
  Net(String, YCoordinate),
}