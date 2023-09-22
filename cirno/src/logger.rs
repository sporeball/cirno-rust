use std::sync::RwLock;

pub static LOG_STATE: RwLock<Vec<String>> = RwLock::new(vec![]);

pub fn debug<T: std::fmt::Debug>(x: &T) {
  let s = format!("{:#?}", &x);
  for line in s.split("\n").collect::<Vec<&str>>() {
    LOG_STATE.write().expect("write to LOG_STATE failed").push(line.to_string());
    // execute!(stdout(), crossterm::style::Print(format!("{}\n", line)));
  }
}
