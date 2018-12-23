use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::store;
use crate::tools;

pub fn run(arc_db: Arc<Mutex<store::DB>>) {
    let host = "127.0.0.1";
    let port = 30160;
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).expect("socket bind failed");
    println!("H2o KV statted at {}", &addr);

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let clone_arc = arc_db.clone();
                thread::spawn(move || {
                    handle_client(&mut stream, clone_arc);
                });
            }
            Err(e) => panic!(e),
        }
    }
}

fn handle_get(data: &[u8], stream: &mut TcpStream, arc_db: Arc<Mutex<store::DB>>) -> bool {
    let size = tools::bytes_to_u16(&data[3..]);
    let mut buffer = Vec::with_capacity(size as usize);
    for _ in 0..size {
        buffer.push(0_u8);
    }
    match stream.read_exact(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full key bytes: {:?}", e);
            return false;
        }
    }

    println!("data: {:?}", data);
    println!("key buffer: {:?}", buffer);
    println!("size: {:?}", size);
    match str::from_utf8(&buffer) {
        Ok(key) => {
            println!("key: {:?}", key);
            let db = arc_db.lock().unwrap();
            if let Some(x) = store::get(key, &db) {
                stream.write(b"\x0c\x00").unwrap();
                let (count, len_buffer) = tools::u64_to_bytes(x.len() as u64);
                stream.write(&[count]).unwrap();
                stream.write(&len_buffer).unwrap();
                stream.write(x.as_bytes()).unwrap();
            } else {
                stream.write(b"\x0c\x02").unwrap();
            }
        }
        Err(e) => {
            println!("from_utf8 failed: {:?}", e);
            return false;
        }
    }

    true
}

fn handle_client(stream: &mut TcpStream, arc_db: Arc<Mutex<store::DB>>) {
    println!("client accepted");

    loop {
        let mut data = [0; 5];
        match stream.read_exact(&mut data) {
            Ok(_) => {
                if data[0] != 0x0c {
                    println!("invalid query header");
                    break;
                }
                if data[2] != 0x00 {
                    println!("currently only plain text is supported");
                    continue;
                }
            }
            Err(_) => {
                println!("bad data received from client, disconnect it");
                break;
            }
        }

        match data[1] {
            0x01 => {
                handle_get(&data, stream, arc_db.clone());
                continue;
            }
            _ => {
                // unknown command
                if let Err(_) = stream.write(b"\x0c\xff") {
                    break;
                }
                continue;
            }
        }
    }

    println!("client disconnected")
}
