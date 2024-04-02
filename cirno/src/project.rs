use crate::{error::CirnoError, terminal::{assert_is_within_bounds_unchecked, move_within_bounds, EventResult}, CirnoState};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::stdout;
use crossterm::{execute, style::Color};
use enum_dispatch::enum_dispatch;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vector2 {
  pub x: u16,
  pub y: u16,
}

impl Debug for Vector2 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    f.write_fmt(format_args!("\u{1b}[36m({}, {})\u{1b}[0m", self.x, self.y))
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Region {
  pub position: Vector2,
  pub size: Vector2,
}

impl Region {
  /// Return whether a region is overlapping another.
  pub fn overlapping(&self, other: &Region) -> bool {
    let r1_left_side = self.position.x;
    let r1_right_side = self.position.x + self.size.x - 1;
    let r1_top_side = self.position.y;
    let r1_bottom_side = self.position.y + self.size.y - 1;
    let r2_left_side = other.position.x;
    let r2_right_side = other.position.x + other.size.x - 1;
    let r2_top_side = other.position.y;
    let r2_bottom_side = other.position.y + other.size.y - 1;
    r1_left_side <= r2_right_side && r1_right_side >= r2_left_side && r1_top_side <= r2_bottom_side && r1_bottom_side >= r2_top_side
  }
  /// Return whether a region is overlapping a Vector2.
  pub fn overlapping_vec2(&self, vec2: Vector2) -> bool {
    self.overlapping(&Region { position: vec2, size: Vector2 { x: 1, y: 1 } })
  }
}

#[derive(Clone, Default)]
// a value that a pin can have
pub enum Value {
  And(Vec<String>),
  Gnd,
  #[default]
  None,
  Or(Vec<String>),
  Vcc,
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Value::And(values) => {
        f.write_str("\u{1b}[33mAnd(\u{1b}[0m")?;
        values.fmt(f)?;
        f.write_str("\u{1b}[33m)\u{1b}[0m")?;
      },
      Value::Or(values) => {
        f.write_str("\u{1b}[33mOr(\u{1b}[0m")?;
        values.fmt(f)?;
        f.write_str("\u{1b}[33m)\u{1b}[0m")?;
      }
      Value::Vcc => {
        f.write_str("\u{1b}[31mVcc\u{1b}[0m")?;
      },
      Value::Gnd => {
        f.write_str("\u{1b}[34mGnd\u{1b}[0m")?;
      },
      Value::None => {
        f.write_str("\u{1b}[90mNone\u{1b}[0m")?;
      },
    }
    Ok(())
  }
}

#[derive(Clone, Default)]
pub enum Voltage {
  #[default]
  Floating,
  High,
  Low,
}

impl Debug for Voltage {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Voltage::High => f.write_str("\u{1b}[31mHigh\u{1b}[0m")?,
      Voltage::Low => f.write_str("\u{1b}[34mLow\u{1b}[0m")?,
      Voltage::Floating => f.write_str("Floating")?,
    };
    Ok(())
  }
}

#[derive(Clone, Debug)]
// an attribute that an object can have
pub enum Attribute {
  Bounds(Vector2),
  Color(Color),
  From(Vector2),
  Label(String),
  Num(u16),
  Position(Vector2),
  To(Vector2),
  Type(String),
  Value(Value),
  YCoordinate(u16),
}

impl std::fmt::Display for Attribute {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "{:?}", self)
  }
}

#[derive(Clone)]
#[enum_dispatch]
pub enum ObjectEnum {
  Chip(Chip),
  Meta(Meta),
  Net(Net),
  Pin(Pin),
  Wire(Wire),
}

impl Debug for ObjectEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    f.write_str("ObjectEnum::")?;
    match self {
      ObjectEnum::Chip(chip) => chip.fmt(f),
      ObjectEnum::Meta(meta) => meta.fmt(f),
      ObjectEnum::Net(net) => net.fmt(f),
      ObjectEnum::Pin(pin) => pin.fmt(f),
      ObjectEnum::Wire(wire) => wire.fmt(f),
    }
  }
}

#[enum_dispatch(ObjectEnum)]
pub trait Object: Debug {
  // fn new() -> Self;
  fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError>;
  fn get_region(&self) -> Option<&Region>;
  fn set_region_size(&mut self, state: &CirnoState) -> Result<(), anyhow::Error>;
  fn get_char(&self) -> Option<(char, crossterm::style::Color)>;
  fn verify(&self, state: &CirnoState) -> Result<(), CirnoError>;
  fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error>;
  fn report(&self, state: &CirnoState) -> Result<(String, crossterm::style::Color), anyhow::Error>;
}

#[derive(Clone, Debug, Default)]
pub struct Chip {
  pub t: String,
  pub region: Region,
}

impl Object for Chip {
  fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Type(t) => self.t = t,
      Attribute::Position(vec2) => self.region.position = vec2,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "chip".to_string())),
    }
    Ok(())
  }
  fn get_region(&self) -> Option<&Region> {
    Some(&self.region)
  }
  fn set_region_size(&mut self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let pins = state.cic_data.get(&self.t).unwrap();
    let width = u16::try_from(pins.len() / 2).unwrap();
    self.region.size = Vector2 { x: width, y: 3 };
    Ok(())
  }
  fn get_char(&self) -> Option<(char, crossterm::style::Color)> {
    None
  }
  fn verify(&self, state: &CirnoState) -> Result<(), CirnoError> {
    // chip type
    if self.t.eq("") {
      return Err(CirnoError::MissingAttribute("chip type".to_string()))
    }
    // TODO: chip position
    // bounds check
    assert_is_within_bounds_unchecked(self.region.position.x, self.region.position.y, state)?;
    assert_is_within_bounds_unchecked(self.region.position.x + self.region.size.x - 1, self.region.position.y + self.region.size.y - 1, state)?;
    Ok(())
  }
  fn render(&self, _state: &CirnoState) -> Result<(), anyhow::Error> {
    Ok(())
  }
  fn report(&self, _state: &CirnoState) -> Result<(String, crossterm::style::Color), anyhow::Error> {
    Ok((String::new(), crossterm::style::Color::White))
  }
}

#[derive(Clone, Debug, Default)]
pub struct Meta {
  pub bounds: Vector2,
}

impl Object for Meta {
  fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Bounds(vec2) => self.bounds = vec2,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "meta".to_string())),
    }
    Ok(())
  }
  fn get_region(&self) -> Option<&Region> {
    None
  }
  fn set_region_size(&mut self, _state: &CirnoState) -> Result<(), anyhow::Error> {
    Ok(())
  }
  fn get_char(&self) -> Option<(char, crossterm::style::Color)> {
    None
  }
  fn verify(&self, _state: &CirnoState) -> Result<(), CirnoError> {
    // bounds
    if self.bounds.x == 0 && self.bounds.y == 0 {
      return Err(CirnoError::MissingAttribute("bounds".to_string()))
    }
    if self.bounds.x == 0 || self.bounds.y == 0 {
      return Err(CirnoError::NamelessInvalidValueForAttribute("bounds".to_string()))
    }
    Ok(())
  }
  fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let bound_x = state.meta.bounds.x;
    let bound_y = state.meta.bounds.y;
    let center_x = state.columns / 2;
    let center_y = state.rows / 2;
    let min_x = center_x - (bound_x / 2) - 1;
    let min_y = center_y - (bound_y / 2) - 1;
    // let max_x = center_x + (bound_x / 2);
    let max_y = center_y + (bound_y / 2);
    execute!(stdout(), crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey))?;
    // top border
    execute!(stdout(), crossterm::cursor::MoveTo(min_x, min_y))?;
    execute!(stdout(), crossterm::style::Print("~".repeat((bound_x + 2).into())))?;
    // side borders
    let mut i = 1;
    while i < bound_y + 1 {
      execute!(stdout(), crossterm::cursor::MoveTo(min_x, min_y + i))?;
      execute!(stdout(), crossterm::style::Print(format!("{}{}{}", "~", " ".repeat(bound_x.into()), "~")))?;
      i = i + 1;
    }
    // bottom border
    execute!(stdout(), crossterm::cursor::MoveTo(min_x, max_y))?;
    execute!(stdout(), crossterm::style::Print("~".repeat((bound_x + 2).into())))?;
    execute!(stdout(), crossterm::style::ResetColor)?;
    Ok(())
  }
  fn report(&self, _state: &CirnoState) -> Result<(String, crossterm::style::Color), anyhow::Error> {
    Ok((String::new(), crossterm::style::Color::White))
  }
}

#[derive(Clone, Debug, Default)]
pub struct Net {
  pub t: String,
  pub region: Region,
}

impl Object for Net {
  fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Type(t) => self.t = t,
      Attribute::YCoordinate(y) => self.region.position.y = y,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "net".to_string())),
    }
    Ok(())
  }
  fn get_region(&self) -> Option<&Region> {
    Some(&self.region)
  }
  fn set_region_size(&mut self, state: &CirnoState) -> Result<(), anyhow::Error> {
    self.region.size = Vector2 { x: state.meta.bounds.x, y: 1 };
    Ok(())
  }
  fn get_char(&self) -> Option<(char, crossterm::style::Color)> {
    match self.t.as_str() {
      "vcc" => Some(('+', crossterm::style::Color::Red)),
      "gnd" => Some(('-', crossterm::style::Color::Blue)),
      _ => unreachable!(),
    }
  }
  fn verify(&self, state: &CirnoState) -> Result<(), CirnoError> {
    // net type
    match self.t.as_str() {
      "vcc" | "gnd" => {},
      "" => return Err(CirnoError::MissingAttribute("net type".to_string())),
      t => return Err(CirnoError::InvalidValueForAttribute(t.to_string(), "net type".to_string())),
    };
    // TODO: net y
    // bounds check
    assert_is_within_bounds_unchecked(0, self.region.position.y, state)?;
    Ok(())
  }
  fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let y = self.region.position.y;
    let bound_x = state.meta.bounds.x;
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
  fn report(&self, _state: &CirnoState) -> Result<(String, crossterm::style::Color), anyhow::Error> {
    match self.t.as_str() {
      "vcc" => Ok(("vcc net".to_string(), crossterm::style::Color::Red)),
      "gnd" => Ok(("gnd net".to_string(), crossterm::style::Color::Blue)),
      _ => unreachable!(),
    }
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
  pub value: Value,
  pub region: Region,
  pub voltage: Voltage,
}

impl Pin {
  pub fn set_temp_region_position(&mut self, index: usize, width: usize) -> Result<(), anyhow::Error> {
    if index >= width {
      let index = u16::try_from(index).unwrap();
      let max = u16::try_from(width).unwrap() * 2;
      self.region.position = Vector2 { x: max - index - 1, y: 0 };
    } else {
      let index = u16::try_from(index).unwrap();
      self.region.position = Vector2 { x: index, y: 2 };
    }
    Ok(())
  }
  pub fn calculate_voltage_from_value(&mut self, voltages: &HashMap<String, Voltage>) -> Result<Voltage, anyhow::Error> {
    match &self.value {
      Value::And(labels) => {
        for label in labels {
          let v = voltages.get(label);
          match v {
            Some(&Voltage::High) => { continue },
            _ => return Ok(Voltage::Low),
          };
        }
        Ok(Voltage::High)
      },
      Value::Or(labels) => {
        for label in labels {
          let v = voltages.get(label);
          match v {
            Some(&Voltage::High) => return Ok(Voltage::High),
            _ => { continue },
          };
        }
        Ok(Voltage::Low)
      },
      _ => Ok(Voltage::Floating),
    }
  }
}

impl Object for Pin {
  fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Label(label) => self.label = label,
      // Attribute::Position(vec2) => self.region.position = vec2,
      Attribute::Value(value) => self.value = value,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "pin".to_string())),
    }
    Ok(())
  }
  fn get_region(&self) -> Option<&Region> {
    Some(&self.region)
  }
  fn set_region_size(&mut self, _state: &CirnoState) -> Result<(), anyhow::Error> {
    self.region.size = Vector2 { x: 1, y: 1 };
    Ok(())
  }
  fn get_char(&self) -> Option<(char, crossterm::style::Color)> {
    Some(('.', crossterm::style::Color::White))
  }
  fn verify(&self, state: &CirnoState) -> Result<(), CirnoError> {
    // TODO: pin position
    // bounds check
    assert_is_within_bounds_unchecked(self.region.position.x, self.region.position.y, state)?;
    Ok(())
  }
  fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let x = self.region.position.x;
    let y = self.region.position.y;
    // rendering
    move_within_bounds(x, y, state)?;
    execute!(stdout(), crossterm::style::Print("."))?;
    Ok(())
  }
  fn report(&self, _state: &CirnoState) -> Result<(String, crossterm::style::Color), anyhow::Error> {
    let (report, color);
    // basic value
    if self.label.ne("") {
      (report, color) = (self.label.to_string(), crossterm::style::Color::Cyan);
    } else {
      (report, color) = match self.value {
        Value::Gnd => ("gnd".to_string(), crossterm::style::Color::Blue),
        Value::Vcc => ("vcc".to_string(), crossterm::style::Color::Red),
        _ => (String::new(), crossterm::style::Color::White),
      };
    }
    // voltage
    match self.voltage {
      Voltage::High => Ok((format!("{} ({})", report, "hi"), color)),
      Voltage::Low => Ok((format!("{} ({})", report, "lo"), color)),
      Voltage::Floating => Ok((report, color)),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Wire {
  pub color: Color,
  pub from: Vector2,
  pub to: Vector2,
  pub label: char,
}

impl Wire {
  pub fn is_connected_to(&self, position: Vector2) -> bool {
    self.from == position || self.to == position
  }
}

impl Default for Wire {
  fn default() -> Wire {
    Wire {
      color: Color::Red,
      from: Vector2::default(),
      to: Vector2::default(),
      label: 'a',
    }
  }
}

impl Object for Wire {
  fn apply_attribute(&mut self, attribute: Attribute) -> Result<(), CirnoError> {
    match attribute {
      Attribute::Color(color) => self.color = color,
      Attribute::From(vec2) => self.from = vec2,
      Attribute::To(vec2) => self.to = vec2,
      a => return Err(CirnoError::InvalidAttributeForObject(a, "wire".to_string()))
    }
    Ok(())
  }
  fn get_region(&self) -> Option<&Region> {
    None
  }
  fn set_region_size(&mut self, _state: &CirnoState) -> Result<(), anyhow::Error> {
    Ok(())
  }
  fn get_char(&self) -> Option<(char, crossterm::style::Color)> {
    Some((self.label, self.color))
  }
  fn verify(&self, state: &CirnoState) -> Result<(), CirnoError> {
    // from and to
    if self.from == self.to {
      return Err(CirnoError::InvalidWire)
    }
    // bounds check
    assert_is_within_bounds_unchecked(self.from.x, self.from.y, state)?;
    assert_is_within_bounds_unchecked(self.to.x, self.to.y, state)?;
    Ok(())
  }
  fn render(&self, state: &CirnoState) -> Result<(), anyhow::Error> {
    let (from_x, from_y) = (self.from.x, self.from.y);
    let (to_x, to_y) = (self.to.x, self.to.y);
    // rendering
    execute!(stdout(), crossterm::style::SetForegroundColor(self.color))?;
    move_within_bounds(from_x, from_y, state)?;
    execute!(stdout(), crossterm::style::Print(self.label))?;
    move_within_bounds(to_x, to_y, state)?;
    execute!(stdout(), crossterm::style::Print(self.label))?;
    execute!(stdout(), crossterm::style::ResetColor)?;
    Ok(())
  }
  fn report(&self, _state: &CirnoState) -> Result<(String, Color), anyhow::Error> {
    Ok((String::new(), crossterm::style::Color::White))
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
  pub arg_commands: HashMap<String, fn(Vec<&str>, &mut CirnoState) -> Result<EventResult, anyhow::Error>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Modes {
  Console,
  Normal,
}
