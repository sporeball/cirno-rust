use std::sync::RwLock;

#[derive(Debug)]
pub enum Level {
  Error,
  Warn,
  Info,
  Debug,
}

impl Level {
  fn marker(&self) -> &str {
    match self {
      Level::Error => "\u{1b}[31m[e]\u{1b}[0m",
      Level::Warn => "\u{1b}[33m[w]\u{1b}[0m",
      Level::Info => "\u{1b}[36m[i]\u{1b}[0m",
      Level::Debug => "\u{1b}[90m[d]\u{1b}[0m",
    }
  }
}

#[derive(Debug)]
pub struct LogItem {
  pub level: Level,
  pub lines: Vec<String>,
}

pub static LOG_STATE: RwLock<Vec<LogItem>> = RwLock::new(vec![]);

fn push(item: LogItem) {
  LOG_STATE.write()
    .expect("write to LOG_STATE failed")
    .push(item);
}

fn log(s: String, level: Level) {
  let s = format!("{} {}", level.marker(), s);
  let lines = s.split("\n").collect::<Vec<&str>>().iter().map(|l| l.to_string()).collect();
  let item = LogItem { level, lines };
  push(item);
}

pub fn error(s: String) {
  log(s, Level::Error);
}

pub fn warn(s: String) {
  log(s, Level::Warn);
}

pub fn info(s: String) {
  log(s, Level::Info);
}

pub fn debug(s: String) {
  log(s, Level::Debug);
}
