pub mod normal;

pub fn switch_to_mode(mode: crate::project::Mode) {
  (mode.event_loop)();
}
