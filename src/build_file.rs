use pulldown_cmark::{html, Options, Parser};
use std::fs;

pub fn build_file(root_dir: &str, path: &str, template: &String) {
  let file = fs::read_to_string(&*format!("{}/{}", &root_dir, &path))
    .unwrap()
    .to_string();

  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TABLES);
  let parser = Parser::new_ext(&*file, options);

  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  let templated = template.replace("{body}", &*html_output);

  if path.ends_with("index.md") {
    fs::create_dir_all(&*format!(
      "{}/build/{}",
      &root_dir,
      &path[6..&path.len() - 8]
    ))
    .ok();
    fs::write(
      &*format!("{}/build/{}.html", &root_dir, &path[6..&path.len() - 3]),
      &templated,
    )
    .ok();
  } else {
    fs::create_dir_all(&*format!(
      "{}/build/{}",
      &root_dir,
      &path[6..&path.len() - 3]
    ))
    .ok();
    fs::write(
      &*format!(
        "{}/build/{}/index.html",
        &root_dir,
        &path[6..&path.len() - 3]
      ),
      &templated,
    )
    .ok();
  }
}
