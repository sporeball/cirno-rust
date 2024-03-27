// need to use "cirno" in this file, not "crate"

use cirno::{CirnoState, read, error::try_to, parser, project::{Meta, Modes, Vector2}};
use std::cell::RefCell;
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
  state.objects = Rc::new(RefCell::new(parser::parse(contents)?));
  state.meta = state.find_meta()?;

  state.set_cic_data()?;
  state.set_region_sizes()?;
  state.set_wire_labels()?;

  let now = Instant::now();
  state.verify()?;
  let elapsed = now.elapsed();
  cirno::logger::info(format!("verified in {:?}", elapsed));

  state.convert_chips()?;
  state.set_pin_voltages()?;
  state.calculate_voltages_from_values()?;

  // cirno::logger::debug(format!("objects: {:?}", &state.objects));
  // cirno::logger::debug(format!("cic_data: {:?}", &state.cic_data));

  let now = Instant::now();
  state.render()?;
  let elapsed = now.elapsed();
  // bar::message(format!("{:?}", elapsed), &state)?;
  cirno::logger::info(format!("rendered in {:?}", elapsed));

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
    objects: Rc::new(RefCell::new(vec![])),
    meta: Meta::default(),
    error: String::new(),
    cic_data: HashMap::new(),
    repeat_amount: 0,
  };

  cirno::terminal::enter()?;

  let args = Cli::parse();
  let filename = args.filename.to_str().unwrap();
  let contents = try_to(read(filename), &mut state)?;
  // unwrap contents, yielding an empty string if the read failed
  let contents_binding = contents.unwrap_or(String::new());
  // only open project if the read succeeded
  if contents_binding.ne("") {
    cirno::logger::info(format!("opening {}", filename));
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
