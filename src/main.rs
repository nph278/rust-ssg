use std::env;
mod build;
mod build_dir;
mod build_file;
mod copy_static;
mod dev;
mod init;
mod print;

fn main() {
    const HELP_STRING: &str =
        "Use `rust-ssg build` to build out the site, and `rust-ssg dev` to start the server.";

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print::info(HELP_STRING);
        print::error("No command specified.");
    } else {
        let command = &args[1];
        if command == "build" {
            build::build();
        } else if command == "init" {
            init::init();
        } else if command == "dev" {
            dev::dev();
        } else {
            print::info(HELP_STRING);
            print::error(&*format!("Unknown command: {}", command));
        }
    }
}
