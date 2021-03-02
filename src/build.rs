use crate::build_dir;
use crate::print;
use std::env;

pub fn build() {
  print::info("Building site...");
  let root_dir_pathbuf = env::current_dir().unwrap();
  let root_dir = root_dir_pathbuf.to_str().unwrap();

  build_dir::build_dir(&root_dir);
}
