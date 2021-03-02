use crate::build_file;
use crate::print;
use glob::glob;
use std::fs;

pub fn build_dir(root_dir: &str) {
  let template = fs::read_to_string(&*format!("{}/template.html", &root_dir))
    .unwrap()
    .to_string();
  fs::remove_dir_all(&*format!("{}/build", root_dir)).ok();
  fs::create_dir(&*format!("{}/build", root_dir)).ok();
  for entry in glob("pages/**/*.md").unwrap() {
    let unwrapped = entry.unwrap();
    let path = unwrapped.to_str().unwrap();
    print::info(&*format!("Building: {}", &path));
    build_file::build_file(&root_dir, &path, &template);
  }
}
