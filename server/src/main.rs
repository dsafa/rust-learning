extern crate server;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use server::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  let a = ThreadPool::new(1);
  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1.\r\n";

  if buffer.starts_with(get) {
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",
    r#"<!DOCTYPE html>
  <html lang="en">
    <head>
      <meta charset="utf-8">
      <title>Hello!</title>
    </head>
    <body>
      <h1>Hello!</h1>
      <p>Hi from Rust</p>
    </body>
  </html>"#);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }
}