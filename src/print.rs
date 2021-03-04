use colored::*;

pub fn error(text: &str) {
  println!(
    "{} {}",
    colored::ColoredString::from("ERROR!").color("red"),
    colored::ColoredString::from(text).color("red")
  );
}

pub fn info(text: &str) {
  println!(
    "{} {}",
    colored::ColoredString::from("INFO:").color("cyan"),
    colored::ColoredString::from(text).color("cyan")
  );
}

// pub fn warning(text: &str) {
//   println!(
//     "{} {}",
//     colored::ColoredString::from("WARNING!").color("yellow"),
//     colored::ColoredString::from(text).color("yellow")
//   );
// }
