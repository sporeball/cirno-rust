// need to use "cirno" in this file, not "crate"

use cirno::{parser, project::Modes, terminal::KeyEventResult, modes::CirnoContext};
use std::io::{self};
use clap::Parser;
use crossterm::event::Event;

/// Full-featured circuit design tool
#[derive(Parser)]
struct Cli {
  filename: std::path::PathBuf,
}

pub struct CirnoState {
  mode: cirno::project::Modes,
  context: cirno::modes::CirnoContext
}

impl CirnoState {
  pub fn get_mode(&mut self) -> cirno::project::Mode {
    match self.mode {
      cirno::project::Modes::Normal => cirno::modes::normal::get(),
    }
  }
  pub fn event_loop(&mut self) -> Result<(), io::Error> {
    loop {
      match crossterm::event::read()? {
        // key event
        Event::Key(event) => {
          let res: KeyEventResult = (self.get_mode().key_event_cb)(event, &mut self.context);
          if matches!(res, KeyEventResult::Exit) {
            return Ok(())
          }
        },
        _ => (),
      }
    }
  }
}

fn main() -> Result<(), io::Error> {
  let default_panic = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    let _ = cirno::terminal::exit();
    default_panic(info);
  }));

  let mut state = CirnoState {
    mode: Modes::Normal,
    context: CirnoContext {
      cursor_x: 0,
      cursor_y: 0,
    }
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
  for object in objects {
    object.render()?;
  }

  state.event_loop()?;

  cirno::terminal::exit()?;

  cirno::logger::debug(&state.context.cursor_x);
  cirno::logger::debug(&state.context.cursor_y);
  println!("{:#?}", cirno::logger::LOG_STATE.read().unwrap());

  Ok(())
}
