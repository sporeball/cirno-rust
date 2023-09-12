use std::io;
use crossterm::execute;
use crossterm::cursor::*;
use crossterm::event::{Event, KeyCode, KeyEvent, read};
use crossterm::terminal::*;

enum KeyEventResult {
  Ok,
  Err,
  Exit,
}

pub fn enter() {
  crossterm::terminal::enable_raw_mode();
  execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen);
  execute!(io::stdout(), crossterm::cursor::Hide);
}

pub fn exit() {
  execute!(io::stdout(), crossterm::cursor::Show);
  execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
  crossterm::terminal::disable_raw_mode();
}

pub fn event_loop() -> Result<(), io::Error> {
  loop {
    match crossterm::event::read()? {
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
    crossterm::event::KeyEvent { code, modifiers, kind, state } => {
      // exit on Ctrl+C
      if matches!(code, crossterm::event::KeyCode::Char('c')) && modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
        KeyEventResult::Exit
      } else {
        KeyEventResult::Ok
      }
    }
  }
}