use std::fs;

pub fn build_file(root_dir: &str, path: &str, template: &String) {
  let file = fs::read_to_string(&*format!("{}/{}", &root_dir, &path))
    .unwrap()
    .to_string();
  let templated = template.replace("%body%", &*file);
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
  println!("{}", file);
}
