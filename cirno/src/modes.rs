use crate::{CirnoState, project::Modes};

pub mod console;
pub mod normal;

pub fn switch_to_mode(mode: Modes, state: &mut CirnoState) {
  state.mode = mode;
}
