use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

use crate::engine;
use crate::store;

pub fn run(db: &mut store::DB) {
    let host = "127.0.0.1";
    let port = 30160;
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("socket bind failed");
    println!("H2o KV statted at {}", &addr);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                handle_client(stream, db);
            }
            Err(e) => panic!(e),
        }
    }
}

fn handle_client(mut stream: TcpStream, db: &mut store::DB) {
    println!("client accepted");

    let mut buffer = [0; 64];
    loop {
        if let Ok(read) = stream.read(&mut buffer) {
            if read == 0 {
                break;
            }

            // TODO: handle inputs longer than 64 bytes,
            // for now we assume all inputs are inside 64 bytes
            match engine::handle_input(&buffer[0..read], db) {
                Ok(x) => {
                    // since we are testing with telnet, we want to see
                    // str instead of byte integers, thus we use the dirty
                    // from_utf8() (and unwrap() for now).
                    let data = format!("{}\n", &x);
                    if let Err(_) = stream.write(data.as_bytes()) {
                        break;
                    }
                }
                Err(e) => {
                    let data = format!("ERROR: {}\n", e);
                    if let Err(_) = stream.write(data.as_bytes()) {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
    println!("client disconnected")
}
