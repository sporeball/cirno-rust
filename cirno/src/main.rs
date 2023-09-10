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

  parser::parse_cip(filename);

  Ok(())
}