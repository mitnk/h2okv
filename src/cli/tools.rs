use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let mut buf = [0; 8];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u64::<LittleEndian>().expect("read_u64 error");
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut buf = [0; 4];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u32::<LittleEndian>().expect("read_u32 error");
}

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let mut buf = [0; 2];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u16::<LittleEndian>().expect("read_u16 error");
}

pub fn u64_to_bytes(n: u64) -> (u8, Vec<u8>) {
    let mut buffer = vec![];
    buffer.write_u64::<LittleEndian>(n).unwrap();
    let mut count: u8 = 8;
    for x in buffer.iter().rev() {
        if *x == 0 {
            count -= 1
        } else {
            break;
        }
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
