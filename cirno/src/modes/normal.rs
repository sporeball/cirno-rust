use std::io;
use crossterm::event::{Event, KeyEvent};
use crate::terminal::KeyEventResult;

static MODE_NORMAL: crate::project::Mode = crate::project::Mode {
  event_loop,
};

pub fn get() -> crate::project::Mode {
  MODE_NORMAL
}

// TODO: figure out how to avoid repeating this 50 times
fn event_loop() -> Result<(), io::Error> {
  loop {
    match crossterm::event::read()? {
      // key event
      Event::Key(event) => {
        let res: KeyEventResult = handle_key_event(event);
        if matches!(res, KeyEventResult::Exit) {
          return Ok(())
        }
      },
      _ => (),
    }
  }
}

fn handle_key_event(event: KeyEvent) -> KeyEventResult {
  match event {
    crossterm::event::KeyEvent { code, modifiers, kind: _, state: _ } => {
      // exit on Ctrl+C
      if matches!(code, crossterm::event::KeyCode::Char('c')) && modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
        return KeyEventResult::Exit
      }
      KeyEventResult::Ok
    }
  }
}
