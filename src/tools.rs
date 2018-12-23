use std::env;
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

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let mut buf = [0; 8];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u64::<LittleEndian>().expect("read_u64 error");
}

pub fn u32_to_bytes(n: u32) -> [u8; 4] {
    let mut buffer = vec![];
    buffer.write_u32::<LittleEndian>(n).unwrap();
    let mut array = [0; 4];
    array.copy_from_slice(&buffer);
    array
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

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let mut buf = [0; 2];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u16::<LittleEndian>().expect("read_u16 error");
}

pub fn u16_to_bytes(n: u16) -> [u8; 2] {
    let mut buffer = vec![];
    buffer.write_u16::<LittleEndian>(n).unwrap();
    let mut array = [0; 2];
    array.copy_from_slice(&buffer);
    array
}

fn current_dir() -> Option<String> {
    let _current_dir;
    match env::current_dir() {
        Ok(x) => _current_dir = x,
        Err(e) => {
            println!("env current_dir error: {:?}", e);
            return None;
        }
    }
    let current_dir;
    match _current_dir.to_str() {
        Some(x) => current_dir = x,
        None => {
            println!("current_dir to_str error");
            return None;
        }
    }
    Some(current_dir.to_string())
}

pub fn get_db_file() -> Option<String> {
    match current_dir() {
        Some(dir) => Some(format!("{}/{}", dir, "h2okv.data")),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::bytes_to_u64;
    use super::re_contains;
    use super::u16_to_bytes;
    use super::u64_to_bytes;

    #[test]
    fn test_u16_to_bytes() {
        assert_eq!(&u16_to_bytes(0), &[0, 0]);
        assert_eq!(&u16_to_bytes(1), &[1, 0]);
        assert_eq!(&u16_to_bytes(2), &[2, 0]);
        assert_eq!(&u16_to_bytes(256), &[0, 1]);
        assert_eq!(&u16_to_bytes(257), &[1, 1]);
        assert_eq!(&u16_to_bytes(65535), &[0xFF, 0xFF]);
    }

    fn _u64_assert(number: u64, bytes_count: u8, buf: &[u8]) {
        assert_eq!(bytes_to_u64(buf), number);
        let (_count, _buf) = u64_to_bytes(number);
        assert_eq!(_count, bytes_count);
        assert_eq!(&_buf, &buf);
    }

    #[test]
    fn test_u64_to_bytes_and_reverse() {
        _u64_assert(0, 0, &[]);
        _u64_assert(1, 1, &[1]);
        _u64_assert(2, 1, &[2]);
        _u64_assert(0xFF, 1, &[0xFF]);
        _u64_assert(0x100, 2, &[0, 1]);
        _u64_assert(0x101, 2, &[1, 1]);
        _u64_assert(0xFFFF, 2, &[0xFF, 0xFF]);
        _u64_assert(0x10000, 3, &[0, 0, 1]);
        _u64_assert(0x1000000, 4, &[0, 0, 0, 1]);
        _u64_assert(0x1000001, 4, &[1, 0, 0, 1]);
        _u64_assert(0x1010101, 4, &[1, 1, 1, 1]);
        _u64_assert(0x1111111, 4, &[17, 17, 17, 1]);
        _u64_assert(0xFFFFFFFF, 4, &[0xFF, 0xFF, 0xFF, 0xFF]);
        _u64_assert(0xFFFFFFFFFF, 5, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        _u64_assert(0x10101010101, 6, &[1, 1, 1, 1, 1, 1]);
        _u64_assert(0x1010101010101, 7, &[1, 1, 1, 1, 1, 1, 1]);
        _u64_assert(0x101010101010101, 8, &[1, 1, 1, 1, 1, 1, 1, 1]);
        _u64_assert(0x100000000000000, 8, &[0, 0, 0, 0, 0, 0, 0, 1]);
        _u64_assert(0x100000000000001, 8, &[1, 0, 0, 0, 0, 0, 0, 1]);
        _u64_assert(0xFFFFFFFFFFFFFFFF, 8, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_re_contains() {
        assert!(re_contains(r"", ""));
        assert!(re_contains(r"a", "a"));
        assert!(re_contains(r"b", "abc"));
        assert!(!re_contains(r"^b", "abc"));
        assert!(re_contains(r"^a.*c$", "abc"));
        assert!(re_contains(r"^f(.+) b(.*) b(.{2})$", "foo bar baz"));
        assert!(!re_contains(r"^f(.+) b(.*) b(.{3})$", "foo bar baz"));
    }
}
