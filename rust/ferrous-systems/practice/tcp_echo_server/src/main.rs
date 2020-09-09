use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const PORT: &str = "127.0.0.1:7878";

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
  let mut buffer = String::new();
  stream.read_to_string(&mut buffer)?;
  println!("{}", buffer);

  stream.write_all(buffer.as_bytes())?;
  Ok(())
}

fn main() -> io::Result<()> {
  let listener = TcpListener::bind(PORT)?;
  println!("Listening on {}", PORT);

  for stream in listener.incoming() {
    handle_client(stream?)?;
  }

  Ok(())
}
