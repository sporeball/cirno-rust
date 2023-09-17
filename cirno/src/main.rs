// need to use "cirno" in this file, not "crate"

use cirno::parser;
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
    cirno::terminal::exit();
    default_panic(info);
  }));

  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  let project: cirno::project::ParseResult = parser::parse(filename).unwrap();
  // println!("{:#?}", project);

  cirno::terminal::enter();

  let objects = match project {
    cirno::project::ParseResult::Cic(cirno::project::Cic { pins: _ }) => todo!(),
    cirno::project::ParseResult::Cip(cirno::project::Cip { objects }) => objects,
  };
  cirno::logger::debug(&objects);
  for object in objects {
    object.render();
  }

  cirno::terminal::event_loop();

  cirno::terminal::exit();

  // println!("{:#?}", cirno::logger::LOG_STATE.read().unwrap());

  Ok(())
}
