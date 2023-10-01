// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, bar, error::try_to, parser, project::Modes};
use std::io::{self};
use std::time::Instant;
use clap::Parser;
use scopeguard;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
  // call the exit function on drop
  let _guard = scopeguard::guard((), |_| {
    // don't try to exit if already exited (ok)
    if crossterm::terminal::is_raw_mode_enabled().ok() == Some(true) {
      let _ = cirno::terminal::exit();
    }
  });

  let (columns, rows) = crossterm::terminal::size()?;

  let mut state = CirnoState {
    columns,
    rows,
    mode: Modes::Normal,
    bound_x: 0,
    bound_y: 0,
    cursor_x: 0,
    cursor_y: 0,
    objects: vec![],
    errors: vec![],
  };

  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  let project: cirno::project::ParseResult = parser::parse(filename).unwrap();
  // println!("{:#?}", project);

  cirno::terminal::enter()?;

  let objects = match project {
    cirno::project::ParseResult::Cic(cirno::project::Cic { pins: _ }) => todo!(),
    cirno::project::ParseResult::Cip(cirno::project::Cip { objects }) => objects,
  };
  cirno::logger::debug(&objects);
  state.objects = objects;
  try_to(state.apply_meta(), &mut state)?;

  let now = Instant::now();
  try_to(state.render(), &mut state)?;
  let elapsed = now.elapsed();
  // bar::message(format!("{:?} ({:?})", filename, elapsed), &state)?;

  state.event_loop()?; // blocking

  cirno::terminal::exit()?;

  cirno::logger::debug(&state.cursor_x);
  cirno::logger::debug(&state.cursor_y);
  // cirno::logger::debug(&state.errors);
  println!("{:#?}", cirno::logger::LOG_STATE.read().unwrap());

  Ok(())
}
