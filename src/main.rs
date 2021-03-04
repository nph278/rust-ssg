use std::process;
mod build;
mod build_dir;
mod build_file;
mod cli;
mod copy_static;
mod dev;
mod init;
mod print;

fn main() {
    match cli::cli() {
        Ok(_) => {
            process::exit(0);
        }
        Err(_) => {
            process::exit(1);
        }
    }
}
