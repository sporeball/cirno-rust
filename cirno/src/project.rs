enum NetType {
  Vcc,
  Gnd,
}

#[derive(Debug)]
// a value that a pin can have
pub enum Value {
  And(Vec<String>),
  Gnd,
  Vcc,
}

#[derive(Debug)]
// an attribute that an object can have
pub enum Attribute {
  Label(String),
  Position(i32, i32),
  Value(Value),
  YCoordinate(i32),
}

#[derive(Debug)]
// an object that cirno can render
pub enum Object {
  Chip(String, Vec<Attribute>),
  Net(String, Vec<Attribute>),
  Pin(i32, Vec<Attribute>),
}