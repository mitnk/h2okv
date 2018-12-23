use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::persistence;
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

fn handle_del(data: &[u8], stream: &mut TcpStream, arc_db: Arc<Mutex<store::DB>>) -> bool {
    let klen = tools::bytes_to_u16(&data[3..]);
    let mut buf_key = Vec::with_capacity(klen as usize);
    for _ in 0..klen {
        buf_key.push(0_u8);
    }
    match stream.read_exact(&mut buf_key) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full key bytes: {:?}", e);
            return false;
        }
    }

    match str::from_utf8(&buf_key) {
        Ok(key) => {
            let mut db = arc_db.lock().unwrap();
            if let Some(_) = store::delete(key, &mut db) {
                stream.write(b"\x0c\x00").unwrap();
            } else {
                stream.write(b"\x0c\x02").unwrap();
            }
        }
        Err(e) => {
            println!("from_utf8 failed: {:?}", e);
            stream.write(b"\x0c\x01").unwrap();
            return false;
        }
    }
    true
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

    match str::from_utf8(&buffer) {
        Ok(key) => {
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

fn handle_put(data: &[u8], stream: &mut TcpStream, arc_db: Arc<Mutex<store::DB>>) -> bool {
    let klen = tools::bytes_to_u16(&data[3..]);
    let mut buf_key = Vec::with_capacity(klen as usize);
    for _ in 0..klen {
        buf_key.push(0_u8);
    }
    match stream.read_exact(&mut buf_key) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full key bytes: {:?}", e);
            return false;
        }
    }

    let key;
    match str::from_utf8(&buf_key) {
        Ok(x) => key = x,
        Err(e) => {
            println!("from_utf8 failed: {:?}", e);
            return false;
        }
    }

    let mut buf_vllen = [0; 1];
    match stream.read_exact(&mut buf_vllen) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full bytes: {:?}", e);
            return false;
        }
    }

    let mut buf_vlen = Vec::with_capacity(buf_vllen[0] as usize);
    for _ in 0..buf_vllen[0] {
        buf_vlen.push(0_u8);
    }
    match stream.read_exact(&mut buf_vlen) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full len bytes: {:?}", e);
            return false;
        }
    }

    let vlen = tools::bytes_to_u64(&buf_vlen);
    let mut buf_value = Vec::with_capacity(vlen as usize);
    for _ in 0..vlen {
        buf_value.push(0_u8);
    }
    match stream.read_exact(&mut buf_value) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full content bytes: {:?}", e);
            return false;
        }
    }

    let mut db = arc_db.lock().unwrap();
    match store::put(key, &buf_value, &mut db) {
        Ok(_) => {
            stream.write(b"\x0c\x00").unwrap();
        }
        Err(_) => {
            stream.write(b"\x0c\x01").unwrap();
        }
    }
    true
}

fn handle_scan(data: &[u8], stream: &mut TcpStream, arc_db: Arc<Mutex<store::DB>>) -> bool {
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

    match str::from_utf8(&buffer) {
        Ok(key) => {
            let db = arc_db.lock().unwrap();
            let items = store::scan(key, &db);
            let len = items.len();
            if len == 0 {
                stream.write(b"\x0c\x02").unwrap();
                return true;
            }

            let buf_len = tools::u32_to_bytes(len as u32);
            stream.write(b"\x0c\x00").unwrap();
            assert_eq!(buf_len.len(), 4);
            stream.write(&buf_len).unwrap();
            for x in items {
                let klen = x.len();
                assert!(klen <= 0xFFFF);
                let buf_klen = tools::u16_to_bytes(klen as u16);
                assert_eq!(buf_klen.len(), 2);
                stream.write(&buf_klen).unwrap();
                stream.write(x.as_bytes()).unwrap();
            }
        }
        Err(e) => {
            println!("failed when from_utf8: {:?}", e);
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
            }
            0x02 => {
                handle_put(&data, stream, arc_db.clone());
                if let Some(db_file) = tools::get_db_file() {
                    let db = arc_db.lock().unwrap();
                    persistence::save_to_file(&db_file, &db);
                }
            }
            0x03 => {
                handle_del(&data, stream, arc_db.clone());
                if let Some(db_file) = tools::get_db_file() {
                    let db = arc_db.lock().unwrap();
                    persistence::save_to_file(&db_file, &db);
                }
            }
            0x04 => {
                handle_scan(&data, stream, arc_db.clone());
            }
            _ => {
                // unknown command
                if let Err(_) = stream.write(b"\x0c\xff") {
                    break;
                }
            }
        }
    }

    println!("client disconnected")
}
