use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::engine;
use crate::store;

pub fn run(arc_db: Arc<Mutex<store::DB>>) {
    let host = "127.0.0.1";
    let port = 30160;
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("socket bind failed");
    println!("H2o KV statted at {}", &addr);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let clone_arc = arc_db.clone();
                thread::spawn(move || {
                    handle_client(stream, clone_arc);
                });
            }
            Err(e) => panic!(e),
        }
    }
}

fn handle_client(mut stream: TcpStream, arc_db: Arc<Mutex<store::DB>>) {
    println!("client accepted");
    let mut buffer = [0; 64];
    loop {
        if let Ok(read) = stream.read(&mut buffer) {
            if read == 0 {
                break;
            }

            // TODO: handle inputs longer than 64 bytes,
            // for now we assume all inputs are inside 64 bytes
            match engine::handle_input(&buffer[0..read], arc_db.clone()) {
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
