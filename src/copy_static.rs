use glob::glob;
use std::fs;

pub fn copy_static(root_dir: &str) {
  for entry in glob("static/**/*").unwrap() {
    let unwrapped = entry.unwrap();
    let path = unwrapped.to_str().unwrap();

    fs::copy(
      &*format!("{}/static/{}", &root_dir, &path[7..]),
      &*format!("{}/build/{}", &root_dir, &path[7..]),
    )
    .ok();
  }
}
