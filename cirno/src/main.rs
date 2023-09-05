use std::io::{self, prelude::*, BufReader};
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
  let file = File::open(filename)?;
  let contents = BufReader::new(file);

  for line in contents.lines() {
    println!("{}", line.unwrap());
  }

  Ok(())
}