use crate::{CirnoState, open, try_to, bar, error::CirnoError, terminal::{EventResult, backspace, clear_all, move_to, read_line}};
use std::collections::HashMap;
use std::io::stdout;
use std::path::PathBuf;
use crossterm::execute;
use enum_dispatch::enum_dispatch;

// TODO: derive Debug manually like with ObjectEnum?
#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum CommandEnum {
  Open(Open),
  Quit(Quit),
  Splash(Splash),
}

// impl Debug for CommandEnum {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//     f.write_str("CommandEnum::")?;
//     match self {
//       CommandEnum::Open => Open.fmt(f),
//       CommandEnum::Quit => Quit.fmt(f),
//       CommandEnum::Splash => Splash.fmt(f),
//     }
//   }
// }


#[enum_dispatch(CommandEnum)]
pub trait Command {
  fn execute(&self, state: &mut CirnoState) -> Result<EventResult, anyhow::Error>;
}

/// Return a HashMap containing all editor commands.
/// This is called each time an instance of CirnoState is created.
pub fn get_all_commands() -> HashMap<String, fn(Vec<String>) -> CommandEnum> {
  HashMap::from([
    ("open".to_string(), (|args| CommandEnum::Open(Open(args))) as fn(Vec<String>) -> CommandEnum),
    ("q".to_string(), (|args| CommandEnum::Quit(Quit(args))) as fn(Vec<String>) -> CommandEnum),
    ("splash".to_string(), (|args| CommandEnum::Splash(Splash(args))) as fn(Vec<String>) -> CommandEnum),
  ])
}

/// Execute a command entered via the bar.
pub fn read_from_bar(state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
  bar::message(":".to_string(), state)?;
  let line = read_line()?;
  // remove the colon if the command comes back empty
  if line.eq("") {
    backspace()?;
    return Ok(EventResult::Drop);
  // TODO: matches some extra things, e.g. `splashhhhh` for `splash`
  } else if let Some(key) = state.commands.keys().find(|&k| line.starts_with(k)) {
    let cmd = state.commands.get(key).unwrap();
    // split line by spaces and remove the first item to yield just the arguments
    let mut args: Vec<String> = line.split(' ').collect::<Vec<&str>>().iter().map(|l| l.to_string()).collect();
    args.remove(0);
    return (cmd)(args).execute(state);
  } else {
    return Err(CirnoError::InvalidCommand(line).into());
  }
}

/// A command to open a cirno project.
/// Arguments: 1
#[derive(Clone, Debug)]
pub struct Open(pub Vec<String>);

impl Command for Open {
  fn execute(&self, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
    let [filename] = self.0.as_slice() else {
      return Err(CirnoError::ArgumentError(1, self.0.len()).into());
    };
    try_to(open(PathBuf::from(filename), state), state)?;
    Ok(EventResult::Ok)
  }
}

/// A command to quit cirno.
/// Arguments: 0
#[derive(Clone, Debug)]
pub struct Quit(pub Vec<String>);

impl Command for Quit {
  fn execute(&self, _state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
    let [] = self.0.as_slice() else {
      return Err(CirnoError::ArgumentError(0, self.0.len()).into());
    };
    Ok(EventResult::Exit)
  }
}

/// A command to render cirno's splash screen.
/// Arguments: 0
#[derive(Clone, Debug)]
pub struct Splash(pub Vec<String>);

impl Command for Splash {
  fn execute(&self, state: &mut CirnoState) -> Result<EventResult, anyhow::Error> {
    let [] = self.0.as_slice() else {
      return Err(CirnoError::ArgumentError(0, self.0.len()).into());
    };
    if state.columns < 30 || state.rows < 6 {
      return Ok(EventResult::Ok)
    }
    let center_x = state.columns / 2;
    let center_y = state.rows / 2;
    clear_all()?;
    move_to(center_x - 12, center_y - 2)?;
    execute!(stdout(), crossterm::style::Print("\u{1b}[36mcirno\u{1b}[0m"))?;
    move_to(center_x - 12, center_y - 1)?;
    execute!(stdout(), crossterm::style::Print("\u{1b}[90m\"I can go anywhere\u{1b}[0m"))?;
    move_to(center_x - 12, center_y)?;
    execute!(stdout(), crossterm::style::Print("\u{1b}[90mand do anything I want!\"\u{1b}[0m"))?;
    move_to(center_x - 15, center_y + 2)?;
    execute!(stdout(), crossterm::style::Print("type :open \u{1b}[34m<filename>\u{1b}[0m to start"))?;
    move_to(center_x - 15, center_y + 3)?;
    execute!(stdout(), crossterm::style::Print("type :q\u{1b}[34m<Enter>\u{1b}[0m to quit"))?;
    Ok(EventResult::Ok)
  }
}
