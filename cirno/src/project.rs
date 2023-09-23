use std::fmt::Debug;
use std::io;
use std::io::stdout;
use crossterm::execute;

#[derive(Debug)]
pub struct Label {
  pub value: String,
}

#[derive(Debug)]
pub struct Num {
  pub num: i32,
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
  None,
  Vcc,
}

impl Default for Value {
  fn default() -> Value {
    Value::None
  }
}

#[derive(Debug, Default)]
pub struct YCoordinate {
  pub y: i32,
}

#[derive(Debug)]
// an attribute that an object can have
pub enum Attribute {
  Label(String),
  Num(i32),
  Position(Position),
  Type(String),
  Value(Value),
  YCoordinate(i32),
}

impl std::fmt::Display for Attribute {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{:?}", self)
  }
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
      a => panic!("could not apply {:?} to chip", a.to_string()),
    }
  }
  pub fn render(self) -> Result<(), io::Error> {
    let (cols, rows) = crossterm::terminal::size()?;
    let center_x = cols / 2;
    let center_y = rows / 2;
    let chip_x = self.position.x as u16;
    let chip_y = self.position.y as u16;
    // bounds check
    if chip_x + center_x > cols || chip_y + center_y > rows {
      return Ok(());
    }
    // read .cic based on type field
    let filename = format!("../stdlib/{}{}", self.t, ".cic"); // TODO: make this stronger, make sure this doesn't break
    let cic: ParseResult = crate::parser::parse(&filename).unwrap();
    let pins = match cic {
      ParseResult::Cic(crate::project::Cic { pins }) => pins,
      ParseResult::Cip(_) => todo!(),
    };
    // rendering
    for pin in pins {
      execute!(stdout(), crossterm::cursor::MoveTo(chip_x + center_x, chip_y + center_y))?;
      pin.render()?;
    }
    Ok(())
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
      a => panic!("could not apply {:?} to net", a),
    }
  }
  pub fn render(self) -> Result<(), io::Error> {
    let (cols, rows) = crossterm::terminal::size()?;
    let center_y = rows / 2;
    let net_y = self.y as u16;
    // bounds check
    if net_y + center_y > rows {
      return Ok(());
    }
    // rendering
    execute!(stdout(), crossterm::cursor::MoveTo(0, net_y + center_y))?;
    if self.t.eq("vcc") {
      execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Red))?;
      execute!(stdout(), crossterm::style::Print("+".repeat(cols.into())))?;
    } else {
      execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::Blue))?;
      execute!(stdout(), crossterm::style::Print("-".repeat(cols.into())))?;
    }
    execute!(stdout(), crossterm::style::ResetColor)?; // TODO: what happens if you don't do this?
    Ok(())
  }
}

impl Default for Net {
  fn default() -> Net {
    Net { t: String::new(), y: -1 }
  }
}

#[derive(Debug, Default)]
pub struct Pin {
  pub label: String,
  pub num: i32,
  pub position: Position,
  pub value: Value,
}

impl Pin {
  pub fn apply_attribute(&mut self, attribute: Attribute) {
    match attribute {
      Attribute::Label(label) => self.label = label,
      Attribute::Num(num) => self.num = num,
      Attribute::Position(position) => self.position = position,
      Attribute::Value(value) => self.value = value,
      a => panic!("could not apply {:?} to chip", a),
    }
  }
  pub fn render(self) -> Result<(), io::Error> {
    let (cols, rows) = crossterm::terminal::size()?;
    let (col, row) = crossterm::cursor::position()?;
    let x = self.position.x as u16;
    let y = self.position.y as u16;
    // bounds check
    if col + x > cols || row + y > rows {
      return Ok(());
    }
    // rendering
    execute!(stdout(), crossterm::cursor::MoveTo(col + x, row + y))?;
    execute!(stdout(), crossterm::style::Print("."))?;
    Ok(())
  }
}

// an object that cirno can render
pub enum Object {
  Chip(Chip),
  Net(Net),
  Pin(Pin),
}

impl Object {
  pub fn apply_attribute(&mut self, attribute: Attribute) {
    match self {
      Object::Chip(chip) => chip.apply_attribute(attribute),
      Object::Net(net) => net.apply_attribute(attribute),
      Object::Pin(pin) => pin.apply_attribute(attribute),
    }
  }
  pub fn render(self) -> Result<(), io::Error> {
    match self {
      Object::Chip(chip) => chip.render(),
      Object::Net(net) => net.render(),
      Object::Pin(pin) => pin.render(),
    }
  }
}

impl Debug for Object {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str("Object::")?;
    match self {
      Object::Chip(chip) => chip.fmt(f),
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
}

impl ParseResult {
  pub fn apply(&mut self, ast: Vec<Object>) {
    match self {
      ParseResult::Cic(cic) => cic.pins = ast,
      ParseResult::Cip(cip) => cip.objects = ast,
    }
  }
  pub fn verify(&mut self) {
    match self {
      ParseResult::Cic(_cic) => todo!(),
      ParseResult::Cip(_cip) => todo!(),
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

#[derive(Clone, Copy)]
pub struct Mode {
  pub key_event_cb: fn(crossterm::event::KeyEvent) -> crate::terminal::KeyEventResult,
}

pub enum Modes {
  Normal,
}