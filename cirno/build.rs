use std::env;
use std::fs::{self, DirEntry, read_to_string};
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
  let out_dir = env::var_os("OUT_DIR").unwrap();
  if dir.is_dir() {
    fs::create_dir_all(
      Path::new(&out_dir)
      .join(dir.strip_prefix("..").unwrap())
    )?;
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        visit_dirs(&path, cb)?;
      } else {
        cb(&entry)?;
      }
    }
  }
  Ok(())
}

fn copy_cic(entry: &DirEntry) -> io::Result<()> {
  let out_dir = env::var_os("OUT_DIR").unwrap();
  let source_path = entry.path();
  let dest_path = Path::new(&out_dir)
    .join(source_path.strip_prefix("..").unwrap().to_str().unwrap());
  fs::write(
    &dest_path,
    read_to_string(source_path).unwrap().as_str()
  ).unwrap();
  Ok(())
}

fn main() -> io::Result<()> {
  visit_dirs(Path::new("../stdlib"), copy_cic)?;
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=../stdlib");
  Ok(())
}