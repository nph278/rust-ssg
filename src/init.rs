use crate::print;
use std::env;
use std::fs;

pub fn init() {
  const DEFAULT_TEMPLATE: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Thingy</title>\n  </head>\n  <body>\n    %body%\n  </body>\n</html>\n";
  const DEFAULT_INDEX: &str = "Hello World! Edit pages/index.md to edit this page!";

  let root_dir_pathbuf = env::current_dir().unwrap();
  let root_dir = root_dir_pathbuf.to_str().unwrap();

  print::info("Initializing a new site.");

  fs::create_dir(&*format!("{}/pages", root_dir)).ok();
  fs::write(&*format!("{}/template.html", &root_dir), DEFAULT_TEMPLATE).ok();
  fs::write(&*format!("{}/pages/index.md", &root_dir), DEFAULT_INDEX).ok();

  print::info("Done initializing. Run `rust-ssg build` to build the site.");
}
