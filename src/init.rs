use crate::print;
use std::env;
use std::fs;

pub fn init() {
  const DEFAULT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="/main.css" />
    <title>Thingy</title>
  </head>
  <body>
    {body}
  </body>
</html>
"#;
  const DEFAULT_INDEX: &str = r#"# Hello World

Edit pages/index.md to edit this page!
"#;
  const DEFAULT_CSS: &str = r#"/* Some CSS */

h1 {
  color: red;
}
"#;

  let root_dir_pathbuf = env::current_dir().unwrap();
  let root_dir = root_dir_pathbuf.to_str().unwrap();

  print::info("Initializing a new site.");

  fs::create_dir(&*format!("{}/pages", &root_dir)).ok();
  fs::create_dir(&*format!("{}/static", &root_dir)).ok();
  fs::write(&*format!("{}/template.html", &root_dir), DEFAULT_TEMPLATE).ok();
  fs::write(&*format!("{}/pages/index.md", &root_dir), DEFAULT_INDEX).ok();
  fs::write(&*format!("{}/static/main.css", &root_dir), DEFAULT_CSS).ok();

  print::info("Done initializing. Run `rust-ssg build` to build the site.");
}
