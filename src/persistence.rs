/// Here we use the `RDB` method[0] like Redis does. To have a higher
/// durable level, `AOF` method[1] should be considerred, which is out scope
/// of our project.
///
/// Basically, we loop on all keys in the HashMap (our data store in memory),
/// for each key, we save it into buffer with content
/// `"\x0C<key-len-byte><key-len-bytes><key-bytes><value-len-byte><value-len-bytes><value-bytes>\x0C<next-key-value-item>"`
/// where `"\x0C"` are one-byte header for future possible feature
/// expansing usage; `<key-len-byte>` is one-byte that how many bytes
/// the next `<key-len-bytes>` used. which store the real key bytes in total
/// the following `<key-bytes>` stored. The value bytes are the same logic.
///
/// [0][1] https://redis.io/topics/persistence
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::store;
use crate::tools;

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

fn get_db_file() -> Option<String> {
    match current_dir() {
        Some(dir) => Some(format!("{}/{}", dir, "h2okv.data")),
        None => None,
    }
}

pub fn save_to_file(db: &store::DB) {
    let file_path = match get_db_file() {
        Some(x) => x,
        None => {
            println!("cannot get db file path");
            return;
        }
    };
    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(why) => {
            println!("couldn't create db file: {:?}", why);
            return;
        }
    };

    let mut buffer: Vec<u8> = Vec::new();
    for (key, value) in db {
        let (count, bytes) = tools::u64_to_bytes(key.len() as u64);
        buffer.push(0x0c_u8); // header
        buffer.push(count.into()); // key-length-bytes count
        buffer.extend(&bytes); // key-length bytes
        buffer.extend(key.as_bytes());
        let (count, bytes) = tools::u64_to_bytes(value.len() as u64);
        buffer.push(count.into()); // value-length-bytes count
        buffer.extend(&bytes); // value-length bytes
        buffer.extend(value.as_bytes());
    }

    if let Err(e) = file.write_all(&buffer) {
        println!("Error when save db: {:?}", e);
    }
}

fn read_buffer(reader: &mut BufReader<File>, buffer: &mut [u8], can_be_empty: bool) -> bool {
    match reader.read(buffer) {
        Ok(n) => {
            if n == 0 {
                if !can_be_empty {
                    panic!("cannot read any data");
                }
                return false;
            }
            true
        }
        Err(e) => {
            panic!("buffer read error: {:?}", e);
        }
    }
}

pub fn load_from_file(arc_db: Arc<Mutex<store::DB>>) {
    let clone_arc = arc_db.clone();
    let mut db = clone_arc.lock().unwrap();

    let file_path = match get_db_file() {
        Some(x) => x,
        None => {
            println!("cannot get db file path");
            return;
        }
    };

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(why) => {
            if why.kind() == ErrorKind::NotFound {
                println!("No existing db file found.");
                return;
            }
            println!("open db file failed: {:?}", why);
            return;
        }
    };

    let mut reader = BufReader::new(file);
    loop {
        // read and confirm the header
        let mut buf_header = [0_u8; 1];
        if !read_buffer(&mut reader, &mut buf_header, true) {
            break; // EOF
        }
        assert_eq!(&buf_header, &[0x0c_u8]);

        // BEGIN of read key
        // 1. read key length byte
        let mut buf_key_len_byte = [0_u8; 1];
        read_buffer(&mut reader, &mut buf_key_len_byte, false);

        // 2. read key length bytes
        let count = buf_key_len_byte[0] as usize;
        let mut buf_key_len = Vec::with_capacity(count);
        for _ in 0..count {
            buf_key_len.push(0_u8);
        }
        read_buffer(&mut reader, &mut buf_key_len, false);

        // 3. read key bytes
        let key_bytes_count = tools::bytes_to_u64(&buf_key_len);
        let mut buf_key = Vec::with_capacity(key_bytes_count as usize);
        for _ in 0..key_bytes_count {
            buf_key.push(0_u8);
        }
        read_buffer(&mut reader, &mut buf_key, false);

        // BEGIN of read value
        // 1. read value length byte
        let mut buf_value_len_byte = [0_u8; 1];
        read_buffer(&mut reader, &mut buf_value_len_byte, false);

        // 2. read value length bytes
        let count = buf_value_len_byte[0] as usize;
        let mut buf_value_len = Vec::with_capacity(count);
        for _ in 0..count {
            buf_value_len.push(0_u8);
        }
        read_buffer(&mut reader, &mut buf_value_len, false);

        // 3. read value bytes
        let value_bytes_count = tools::bytes_to_u64(&buf_value_len);
        let mut buf_value = Vec::with_capacity(value_bytes_count as usize);
        for _ in 0..value_bytes_count {
            buf_value.push(0_u8);
        }
        read_buffer(&mut reader, &mut buf_value, false);

        db.insert(
            String::from_utf8(buf_key).unwrap(),
            String::from_utf8(buf_value).unwrap(),
        );
    }
}
