use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use crate::tools;

pub fn scan(key: &str, stream: &mut TcpStream) {
    // send query
    stream.write(b"\x0c\x04\x00").unwrap();
    let len = key.len();
    assert!(len <= 0xFFFF);
    stream.write(&tools::u16_to_bytes(len as u16)).unwrap();
    if !key.is_empty() {
        stream.write(key.as_bytes()).unwrap();
    }

    // handle response
    let mut data = [0_u8; 2];
    match stream.read_exact(&mut data) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    if data[0] != 0x0c {
        println!("bad header from server");
        return;
    }

    if data[1] == 0x01 {
        println!("query failed");
        return;
    } else if data[1] == 0x02 {
        println!("no such key");
        return;
    } else if data[1] == 0xFF {
        println!("unknown command");
        return;
    }

    if data[1] != 0x00 {
        println!("unknown error code");
        return;
    }

    let mut buf_count = [0; 4];
    match stream.read_exact(&mut buf_count) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full count bytes: {:?}", e);
            return;
        }
    }

    let count = tools::bytes_to_u32(&buf_count);
    for i in 0..count {
        let mut buf_len = [0; 2];
        match stream.read_exact(&mut buf_len) {
            Ok(_) => {}
            Err(e) => {
                println!("cannot read full count bytes: {:?}", e);
                return;
            }
        }

        let size = tools::bytes_to_u16(&buf_len);
        let mut buf_key = Vec::with_capacity(size as usize);
        for _ in 0..size {
            buf_key.push(0_u8);
        }
        match stream.read_exact(&mut buf_key) {
            Ok(_) => {}
            Err(e) => {
                println!("cannot read full content bytes: {:?}", e);
                return;
            }
        }

        match str::from_utf8(&buf_key) {
            Ok(x) => {
                println!("{}) {:?}", i + 1, x);
            }
            Err(e) => {
                println!("ERROR: from_utf8 failed: {:?}", e);
                println!("{}) {:?}", i + 1, &buf_key);
            }
        }
    }
}
