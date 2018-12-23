use std::net::TcpStream;

use linefeed::{Interface, ReadResult};

mod cli;
mod do_get;
mod do_scan;
mod tools;

fn main() {
    let reader = Interface::new("my-application").unwrap();
    reader.set_prompt("h2okv> ").unwrap();

    let host = "127.0.0.1";
    let port = 30160;
    let addr = format!("{}:{}", host, port);

    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            println!("Connected to h2okv server {}, Ctrl-D to exit", &addr);

            while let ReadResult::Input(input) = reader.read_line().unwrap() {
                cli::query(&input, &mut stream);
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

}
