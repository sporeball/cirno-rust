use crossterm::event::KeyEvent;
use crate::terminal::KeyEventResult;

pub const fn get() -> crate::project::Mode {
  crate::project::Mode {
    key_event_cb: handle_key_event,
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
