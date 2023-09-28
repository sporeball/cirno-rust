// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, parser, project::Modes};
use std::io::{self};
use clap::Parser;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

fn main() -> Result<(), io::Error> {
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
  state.apply_meta();
  state.render();

  state.event_loop()?;

  cirno::terminal::exit()?;

  cirno::logger::debug(&state.cursor_x);
  cirno::logger::debug(&state.cursor_y);
  println!("{:#?}", cirno::logger::LOG_STATE.read().unwrap());

  Ok(())
}
