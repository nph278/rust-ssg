use crate::build;
use crate::print;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn dev() {
  print::info("Listing on http://localhost:3000");

  // Thread for building
  thread::spawn(move || {
    build::build();

    let root_dir_pathbuf = env::current_dir().unwrap();
    let root_dir = root_dir_pathbuf.to_str().unwrap();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();
    watcher.watch("./pages", RecursiveMode::Recursive).unwrap();
    watcher.watch("./static", RecursiveMode::Recursive).unwrap();
    watcher
      .watch("./template.html", RecursiveMode::Recursive)
      .unwrap();

    loop {
      match rx.recv() {
        Ok(event) => match &event {
          DebouncedEvent::Write(path) => {
            if !path
              .to_str()
              .unwrap()
              .contains(&*format!("{}/build", root_dir))
            {
              println!("--------------------");
              build::build();
            }
          }
          DebouncedEvent::Create(path) => {
            if !path
              .to_str()
              .unwrap()
              .contains(&*format!("{}/build", root_dir))
            {
              println!("--------------------");
              build::build();
            }
          }
          _ => (),
        },
        Err(_) => (),
      }
    }
  });

  let listener = TcpListener::bind("localhost:3000").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream)
  }
}

fn handle_connection(mut stream: std::net::TcpStream) {
  let root_dir_pathbuf = env::current_dir().unwrap();
  let root_dir = root_dir_pathbuf.to_str().unwrap();

  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let request = String::from(std::str::from_utf8(&buffer).unwrap());
  let split: Vec<&str> = request.split(" ").collect();
  let requested_path = split[1].replace("index.html", "").replace(".html", "");

  let response: String;

  if requested_path.contains(".") {
    let formatted = format!("{}/build{}", root_dir, requested_path).replace("//", "/");
    let file_path = &*formatted;
    let formatted_response = format!(
      "HTTP/1.1 200 OK\r\n\r\n{}",
      match fs::read_to_string(file_path) {
        Ok(f) => f.to_string(),
        Err(_) => String::from(""),
      }
    );
    response = formatted_response;
  } else {
    let formatted = format!("{}/build{}/index.html", root_dir, requested_path).replace("//", "/");
    let file_path = &*formatted;
    let formatted_response = format!(
      "HTTP/1.1 200 OK\r\n\r\n{}",
      match fs::read_to_string(file_path) {
        Ok(f) => f.to_string(),
        Err(_) => String::from(""),
      }
    );
    response = formatted_response;
  }
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
