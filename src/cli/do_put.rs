use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use crate::tools;

pub fn put(key: &str, value: &str, stream: &mut TcpStream) {
    // send query
    stream.write(b"\x0c\x02\x00").unwrap();
    let klen = key.len();
    assert!(klen <= 0xFFFF);
    stream.write(&tools::u16_to_bytes(klen as u16)).unwrap();
    stream.write(key.as_bytes()).unwrap();
    let vlen = value.len();
    let (count, buf_len) = tools::u64_to_bytes(vlen as u64);
    stream.write(&[count]).unwrap();
    stream.write(&buf_len).unwrap();
    stream.write(value.as_bytes()).unwrap();

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
        println!("OK");
    } else if data[1] == 0x01 {
        println!("put failed");
    } else {
        println!("unknown server error");
    }
}
