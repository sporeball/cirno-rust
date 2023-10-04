// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, error::{CirnoError, try_to}, parser, project::{Cic, Cip, Modes, ParseResult}};
// use std::time::Instant;
use clap::Parser;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

fn open(filename: &str, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  let project = try_to(parser::parse(filename), state)?;
  if project.is_none() {
    return Err(CirnoError::CouldNotOpenProject.into());
  }

  state.objects = match project {
    Some(ParseResult::Cic(Cic { pins: _ })) => todo!(),
    Some(ParseResult::Cip(Cip { objects })) => objects,
    None => unreachable!(),
  };

  cirno::logger::debug(&state.objects);
  try_to(state.apply_meta(), state)?;

  // let now = Instant::now();
  try_to(state.render(), state)?;
  // let elapsed = now.elapsed();
  // bar::message(format!("{:?} ({:?})", filename, elapsed), &state)?;

  if state.errors.len() > 0 {
    return Err(CirnoError::CouldNotOpenProject.into());
  }

  Ok(())
}

fn main() -> Result<(), anyhow::Error> {
  let default_panic = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    let _ = cirno::terminal::exit();
    default_panic(info);
  }));

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

  cirno::terminal::enter()?;

  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  // TODO: i wish we didn't have to pass &mut state two times
  try_to(open(filename, &mut state), &mut state)?;

  state.event_loop()?; // blocking

  cirno::terminal::exit()?;

  cirno::logger::debug(&state.cursor_x);
  cirno::logger::debug(&state.cursor_y);
  // cirno::logger::debug(&state.errors);
  println!("{:#?}", cirno::logger::LOG_STATE.read().unwrap());

  Ok(())
}
