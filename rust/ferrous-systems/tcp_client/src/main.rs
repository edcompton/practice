use std::io;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

fn main() -> io::Result<()> {
    let arg = std::env::args().nth(1);

    let message = match arg {
        Some(msg) => msg,
        None => String::from("Default Message"),
    };

    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("No connection to the server");
    let result = stream.write(message.as_bytes())?;
    stream
        .shutdown(Shutdown::Write)
        .expect("Shutdown call failed");
    println!("{:?}", result.to_string());
    Ok(())
}
