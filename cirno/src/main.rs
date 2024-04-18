// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, open, error::try_to, modes::normal::splash, project::{Meta, Modes, Vector2}};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use clap::Parser;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: Option<std::path::PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
  let default_panic = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    let _ = cirno::terminal::exit();
    default_panic(info);
  }));

  let args = Cli::parse();

  let (columns, rows) = crossterm::terminal::size()?;

  let mut state = CirnoState {
    columns,
    rows,
    mode: Modes::Normal,
    cursor: Vector2::default(),
    char_under_cursor: (' ', crossterm::style::Color::White),
    objects: Rc::new(RefCell::new(vec![])),
    meta: Meta::default(),
    error: String::new(),
    cic_data: HashMap::new(),
    repeat_amount: 0,
    search_result: Rc::new(RefCell::new(vec![])),
  };

  cirno::terminal::enter()?;

  match args.filename {
    Some(f) => { try_to(open(f, &mut state), &mut state)?; },
    None => { splash(&mut state)?; },
  };

  // Windows 11 spits out a resize event as soon as cirno starts, which
  // is unneeded and should be dropped
  if crossterm::event::poll(Duration::from_secs(0))? == true {
    crossterm::event::read()?; // drop
  }

  state.event_loop()?; // blocking

  cirno::terminal::exit()?;

  Ok(())
}
