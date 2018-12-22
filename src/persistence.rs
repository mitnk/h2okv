/// Here we use the `RDB` method[0] like Redis does. To have a higher
/// durable level, `AOF` method[1] should be considerred, which is out scope
/// of our project.
///
/// Basically, we loop on all keys in the HashMap (our data store in memory),
/// for each key, we save it into buffer with content
/// `"\x0C<key-len-byte><key-len-bytes><key-bytes><value-len-byte><value-len-bytes><value-bytes>\x0C<next-key-value-bytes>"`
/// where `"\x0C"` are one-byte header for future possible feature
/// expansing usage; `<key-len-byte>` is one-byte that how many bytes
/// the next `<key-len-bytes>` used. which store the real key bytes in total
/// the following `<key-bytes>` stored. The value bytes are the same logic.
///
/// [0][1] https://redis.io/topics/persistence

use std::env;
use std::fs::File;
use std::io::Write;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use crate::store;

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
        Some(dir) => {
            Some(format!("{}/{}", dir, "h2okv.data"))
        }
        None => {
            None
        }
    }
}

fn u64_to_bytes(n: u64) -> (u8, Vec<u8>) {
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
        let (count, bytes) = u64_to_bytes(key.len() as u64);
        buffer.push(0x0c_u8);  // header
        buffer.push(count.into());  // key-length-bytes count
        buffer.extend(&bytes); // key-length bytes
        buffer.extend(key.as_bytes());
        let (count, bytes) = u64_to_bytes(value.len() as u64);
        buffer.push(count.into());  // value-length-bytes count
        buffer.extend(&bytes); // value-length bytes
        buffer.extend(value.as_bytes());
    }

    if let Err(e) = file.write_all(&buffer) {
        println!("Error when save db: {:?}", e);
    }
}
