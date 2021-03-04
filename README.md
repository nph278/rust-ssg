# rust-ssg

A simple static site generator written in rust.

## Installation

### Building from source

Do build from source, you need rust installed. You can install it [here](https://www.rust-lang.org/learn/get-started).

First, clone the repo.

```sh
git clone https://github.com/nph278/rust-ssg.git
```

Then, run `cargo build --release` to build out the project. You can install the build on your computer with `install target/release-rust-ssg usr/local/bin/rust-ssg`.

## Usage

Run `cargo-ssg init` in an empty folder to create a website.

You can now build the default site with `cargo-ssg build`.

To start developing, run `cargo-ssg dev` and go to `localhost:3000` in you browser.
