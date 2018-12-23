use std::env;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// convert bytes to u64, LittleEndian
///
/// bytes are not necessarily to be length 8
pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let mut buf = [0; 8];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u64::<LittleEndian>().expect("read_u64 error");
}

/// Convert u32 to bytes of length 4, LittleEndian
pub fn u32_to_bytes(n: u32) -> [u8; 4] {
    let mut buffer = vec![];
    buffer.write_u32::<LittleEndian>(n).unwrap();
    let mut array = [0; 4];
    array.copy_from_slice(&buffer);
    array
}

/// Convert u64 to bytes of length 4, LittleEndian
///
/// bytes count is equal or less than 8. All right side ZEROs will be
/// removed to save space.
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

/// Convert bytes to u16
pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    let mut buf = [0; 2];
    for (i, x) in bytes.iter().enumerate() {
        buf[i] = *x;
    }
    let mut rdr = Cursor::new(&buf);
    return rdr.read_u16::<LittleEndian>().expect("read_u16 error");
}

/// Convert u16 to bytes of length 2
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
    use super::bytes_to_u16;
    use super::bytes_to_u64;
    use super::u16_to_bytes;
    use super::u32_to_bytes;
    use super::u64_to_bytes;

    #[test]
    fn test_u16_to_bytes() {
        assert_eq!(&u16_to_bytes(0), &[0, 0]);
        assert_eq!(bytes_to_u16(&[0, 0]), 0);
        assert_eq!(&u16_to_bytes(1), &[1, 0]);
        assert_eq!(bytes_to_u16(&[1, 0]), 1);
        assert_eq!(&u16_to_bytes(2), &[2, 0]);
        assert_eq!(bytes_to_u16(&[2, 0]), 2);
        assert_eq!(&u16_to_bytes(256), &[0, 1]);
        assert_eq!(bytes_to_u16(&[0, 1]), 256);
        assert_eq!(&u16_to_bytes(257), &[1, 1]);
        assert_eq!(bytes_to_u16(&[1, 1]), 257);
        assert_eq!(&u16_to_bytes(65535), &[0xFF, 0xFF]);
        assert_eq!(bytes_to_u16(&[0xFF, 0xFF]), 0xFFFF);
    }

    #[test]
    fn test_u32_to_bytes() {
        assert_eq!(&u32_to_bytes(0), &[0, 0, 0, 0]);
        assert_eq!(&u32_to_bytes(1), &[1, 0, 0, 0]);
        assert_eq!(&u32_to_bytes(2), &[2, 0, 0, 0]);
        assert_eq!(&u32_to_bytes(256), &[0, 1, 0, 0]);
        assert_eq!(&u32_to_bytes(257), &[1, 1, 0, 0]);
        assert_eq!(&u32_to_bytes(65535), &[0xFF, 0xFF, 0, 0]);
        assert_eq!(&u32_to_bytes(0x11111111), &[0x11, 0x11, 0x11, 0x11]);
        assert_eq!(&u32_to_bytes(0x12131415), &[0x15, 0x14, 0x13, 0x12]);
        assert_eq!(&u32_to_bytes(0xFFFF0001), &[1, 0, 0xFF, 0xFF]);
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
        _u64_assert(
            0xFFFFFFFFFFFFFFFF,
            8,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        );
    }
}
