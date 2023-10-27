// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, read, bar, error::try_to, parser, project::{Meta, Modes, Vector2}};
use std::rc::Rc;
use std::time::Instant;
use clap::Parser;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

/// Open a cirno project given its contents.
fn open(contents: &str, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  state.objects = Rc::new(parser::parse(contents)?);
  cirno::logger::debug(&state.objects);

  state.apply_meta()?;
  state.verify()?;

  let now = Instant::now();
  state.render()?;
  let elapsed = now.elapsed();
  bar::message(format!("{:?}", elapsed), &state)?;

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
    cursor: Vector2::default(),
    objects: Rc::new(vec![]),
    meta: Meta::default(),
    errors: vec![],
  };

  cirno::terminal::enter()?;

  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  let contents = try_to(read(filename), &mut state)?;
  let contents_binding = contents.unwrap_or(String::new());
  // TODO: i wish we didn't have to pass &mut state two times
  try_to(open(&contents_binding, &mut state), &mut state)?;

  state.event_loop()?; // blocking

  cirno::terminal::exit()?;

  cirno::logger::debug(&state.cursor);
  println!("{:#?}", cirno::logger::LOG_STATE.read().unwrap());

  Ok(())
}
