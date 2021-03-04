use glob::glob;
use std::fs;

pub fn copy_static(root_dir: &str) {
  for entry in glob("static/**/*").unwrap() {
    let unwrapped = entry.unwrap();
    let path = unwrapped.to_str().unwrap();

    let file = fs::read_to_string(&*format!("{}/{}", &root_dir, &path))
      .unwrap()
      .to_string();

    fs::write(&*format!("{}/build/{}", &root_dir, &path[7..]), file).ok();
  }
}
