use crate::build_dir;
use crate::copy_static;
use crate::print;
use std::env;
use std::fs;

pub fn build() {
  let root_dir_pathbuf = env::current_dir().unwrap();
  let root_dir = root_dir_pathbuf.to_str().unwrap();

  print::info("Building site...");

  fs::remove_dir_all(&*format!("{}/build", root_dir)).ok();
  fs::create_dir(&*format!("{}/build", root_dir)).ok();

  build_dir::build_dir(&root_dir);

  print::info("Copying static files...");

  copy_static::copy_static(&root_dir);
}
