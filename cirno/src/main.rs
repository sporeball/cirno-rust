// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, read, bar, error::try_to, parser, project::{Meta, Modes, Vector2}};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};
use clap::Parser;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

/// Open a cirno project given its contents.
fn open(contents: &str, state: &mut CirnoState) -> Result<(), anyhow::Error> {
  state.objects = Rc::new(parser::parse(contents)?);
  cirno::logger::debug(format!("objects: {:#?}", &state.objects));

  state.apply_meta()?;
  state.verify()?;
  state.set_cic_data()?;

  let now = Instant::now();
  state.render()?;
  let elapsed = now.elapsed();
  bar::message(format!("{:?}", elapsed), &state)?;
  cirno::logger::info(format!("finished rendering in {:?}", elapsed));

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
    cic_data: HashMap::new(),
  };

  cirno::terminal::enter()?;

  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  let contents = try_to(read(filename), &mut state)?;
  // unwrap contents, yielding an empty string if the read failed
  let contents_binding = contents.unwrap_or(String::new());
  // only open project if the read succeeded
  if contents_binding.ne("") {
    // TODO: i wish we didn't have to pass &mut state two times
    try_to(open(&contents_binding, &mut state), &mut state)?;
  }

  // Windows 11 spits out a resize event as soon as cirno starts, which
  // is unneeded and should be dropped
  if crossterm::event::poll(Duration::from_secs(0))? == true {
    crossterm::event::read()?; // drop
  }

  state.event_loop()?; // blocking

  cirno::terminal::exit()?;

  Ok(())
}
