use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use regex::Regex;

pub fn re_contains(ptn: &str, text: &str) -> bool {
    let re;
    match Regex::new(ptn) {
        Ok(x) => {
            re = x;
        }
        Err(e) => {
            println!("Regex new failed: {:?}", e);
            return false;
        }
    }
    re.is_match(text)
}

pub fn bytes_to_u64(bytes: &mut Vec<u8>) -> u64 {
    while bytes.len() < 8 {
        bytes.push(0_u8);
    }
    let mut rdr = Cursor::new(&bytes);
    return rdr.read_u64::<LittleEndian>().expect("read_u64 error");
}

pub fn u64_to_bytes(n: u64) -> (u8, Vec<u8>) {
    let mut buffer = vec![];
    buffer.write_u64::<LittleEndian>(n).unwrap();
    let mut count: u8 = 0;
    for x in &buffer {
        if *x == 0 {
            break;
        }
        count += 1;
    }
    buffer.truncate(count as usize);
    return (count, buffer);
}

pub fn u16_to_bytes(n: u16) -> [u8; 2] {
    let mut buffer = vec![];
    buffer.write_u16::<LittleEndian>(n).unwrap();
    let mut array = [0; 2];
    array.copy_from_slice(&buffer);
    array
}
