use std::env;
mod build;
mod build_dir;
mod build_file;
mod copy_static;
mod init;
mod print;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print::error("No command specified.")
    } else {
        let command = &args[1];
        if command == "build" {
            build::build();
        } else if command == "init" {
            init::init();
        } else {
            print::error(&*format!("Unknown command: {}", command));
        }
    }
}
