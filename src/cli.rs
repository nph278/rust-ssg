use crate::build;
use crate::dev;
use crate::init;
use crate::print;
use std::env;

pub fn cli() -> Result<(), ()> {
  const HELP_STRING: &str =
    "Use `rust-ssg build` to build out the site, and `rust-ssg dev` to start the server.";

  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    print::info(HELP_STRING);
    print::error("No command specified.");
    Err(())
  } else {
    let command = &args[1];
    if command == "build" {
      build::build();
      Ok(())
    } else if command == "init" {
      init::init();
      Ok(())
    } else if command == "dev" {
      dev::dev();
      Ok(())
    } else {
      print::info(HELP_STRING);
      print::error(&*format!("Unknown command: {}", command));
      Err(())
    }
  }
}
