// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, open, error::try_to, modes::normal::splash};
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
  let mut state = CirnoState::new()?;

  cirno::terminal::enter()?;

  match args.filename {
    Some(f) => { try_to(open(f, &mut state), &mut state)?; },
    None => { splash(&mut state)?; },
  };

  // Windows 11 spits out a resize event as soon as cirno starts, which
  // is unneeded and should be dropped
  if crossterm::event::poll(Duration::from_secs(0))? {
    crossterm::event::read()?; // drop
  }

  state.event_loop()?; // blocking

  cirno::terminal::exit()?;

  Ok(())
}
