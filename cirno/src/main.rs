// need to use "cirno" in this file, not "crate"

use cirno::parser;
use std::io::{self, BufReader};
use std::fs::File;
use clap::Parser;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

fn main() -> Result<(), io::Error> {
  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  let project: cirno::project::ParseResult = parser::parse(filename).unwrap();
  // println!("{:#?}", project);

  cirno::terminal::enter();
  cirno::terminal::event_loop();
  cirno::terminal::exit();

  Ok(())
}