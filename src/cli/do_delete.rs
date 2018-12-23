use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use crate::tools;

pub fn delete(key: &str, stream: &mut TcpStream) {
    // send query
    stream.write(b"\x0c\x03\x00").unwrap();
    let klen = key.len();
    assert!(klen <= 0xFFFF);
    stream.write(&tools::u16_to_bytes(klen as u16)).unwrap();
    stream.write(key.as_bytes()).unwrap();

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

    if data[1] == 0x00 {
        println!("1");
    } else {
        println!("0");
    }
}
