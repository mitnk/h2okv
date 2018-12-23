use std::io::{self, Write};
use std::net::TcpStream;

mod cli;
mod do_delete;
mod do_get;
mod do_put;
mod do_scan;
mod tools;

fn main() {
    let host = "127.0.0.1";
    let port = 30160;
    let addr = format!("{}:{}", host, port);

    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            println!("Connected to h2okv server {}, Ctrl-D to exit", &addr);

            let stdin = io::stdin();
            let input = &mut String::new();
            loop {
                input.clear();
                print!("h2okv> ");
                io::stdout().flush().unwrap();
                stdin.read_line(input).unwrap();
                cli::query(&input, &mut stream);
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
