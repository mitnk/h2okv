use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use crate::tools;

pub fn get(key: &str, stream: &mut TcpStream) {
    // send query
    stream.write(b"\x0c\x01\x00").unwrap();
    let len = key.len();
    assert!(len <= 0xFFFF);
    stream.write(&tools::u16_to_bytes(len as u16)).unwrap();
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

    if data[1] == 0x01 {
        println!("query failed");
        return;
    } else if data[1] == 0x02 {
        println!("(None)");
        return;
    } else if data[1] == 0xFF {
        println!("unknown command");
        return;
    }

    if data[1] != 0x00 {
        println!("unknown error code");
        return;
    }

    let mut buf_flag = [0; 1];
    match stream.read_exact(&mut buf_flag) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full llen bytes: {:?}", e);
            return;
        }
    }
    if buf_flag[0] != 0x00 {
        println!("currently, only plain text is supported.");
        return;
    }

    let mut buf_llen = [0; 1];
    match stream.read_exact(&mut buf_llen) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full llen bytes: {:?}", e);
            return;
        }
    }

    let mut buffer_len = Vec::with_capacity(buf_llen[0] as usize);
    for _ in 0..buf_llen[0] {
        buffer_len.push(0_u8);
    }
    match stream.read_exact(&mut buffer_len) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full len bytes: {:?}", e);
            return;
        }
    }

    let size = tools::bytes_to_u64(&buffer_len);
    let mut buffer_content = Vec::with_capacity(size as usize);
    for _ in 0..size {
        buffer_content.push(0_u8);
    }
    match stream.read_exact(&mut buffer_content) {
        Ok(_) => {}
        Err(e) => {
            println!("cannot read full content bytes: {:?}", e);
            return;
        }
    }
    match str::from_utf8(&buffer_content) {
        Ok(x) => {
            println!("{:?}", x);
        }
        Err(e) => {
            println!("ERROR: from_utf8 failed: {:?}", e);
            println!("content: {:?}", &buffer_content);
        }
    }
}
