// Here we use the `RDB` method[0] like Redis does. To have a higher
// durable level, `AOF` method[1] should be considerred, which is out scope
// of our project.
//
// [0][1] https://redis.io/topics/persistence
//
// TODO 1: for now, we save the whole db content into disk file for each
//   write query (put/delete). But eventually, we want the way Redis do
//   with `RDB` method: save with a time interval & when exiting process.
//
// TODO 2: We probably want to keep multiple backup disk files, in cases
//   that bad data save into the current single file.
//

use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::{Read, Write};

use crate::store;
use crate::tools;

/// Save current DB content into disk file for persistence.
///
/// The target disk file is the current working directory, with file
/// name: h2okv.data. Currently, each time saving, we destroy & recreate it,
/// which is a temporary solution. For details, please see comments at
/// the top of this file.
///
/// Basically, we loop on all keys in the HashMap (our data store in memory),
/// for each key, we save it into buffer with content
/// `"\x0C<key-len-byte><key-len-bytes><key-bytes><value-len-byte><value-len-bytes><value-bytes>\x0C<next-key-value-item>"`
/// where `"\x0C"` are one-byte header for future possible feature
/// expansing usage; `<key-len-byte>` is one-byte that how many bytes
/// the next `<key-len-bytes>` used. which store the real key bytes in total
/// the following `<key-bytes>` stored. The value bytes are the same logic.
pub fn save_to_file(db_file: &str, db: &store::DB) {
    let mut file = match File::create(db_file) {
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

/// Load existing DB disk file into DB memory.
///
/// The reverse action with `save_to_file()`. For file format, please see
/// comments of `save_to_file()`.
pub fn load_from_file(db_file: &str, db: &mut store::DB) {
    let file = match File::open(db_file) {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::store;
    use super::load_from_file;
    use super::save_to_file;

    #[test]
    fn test_load_from_file_empty() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/data/dataset-001.db");
        let mut db = store::DB::new();
        assert_eq!(store::scan("", &db), Vec::<String>::new());
        load_from_file(d.to_str().unwrap(), &mut db);
        assert_eq!(store::scan("", &db), Vec::<String>::new());
    }

    #[test]
    fn test_load_from_file_single_item() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/data/dataset-002.db");
        let mut db = store::DB::new();
        assert_eq!(store::scan("", &db), Vec::<String>::new());
        load_from_file(d.to_str().unwrap(), &mut db);
        assert_eq!(store::scan("", &db), vec!["foo".to_string()]);
    }

    #[test]
    fn test_load_from_file_multiple_items() {
        let mut db_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        db_file.push("tests/data/dataset-003.db");
        let mut db = store::DB::new();
        assert_eq!(store::scan("", &db), Vec::<String>::new());
        load_from_file(db_file.to_str().unwrap(), &mut db);
        let v = store::scan("", &db);
        assert_eq!(v.len(), 4);
        assert_eq!(store::get("foo", &db), Some("barbaz".to_string()));
        assert_eq!(store::get("lang", &db), Some("Rust".to_string()));
        assert_eq!(store::get("name-en", &db), Some("Hugo".to_string()));
        assert_eq!(store::get("name-cn", &db), Some("宏钢".to_string()));
    }

    #[test]
    fn test_save_to_file() {
        let mut db_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        db_file.push("tests/data/dataset-tmp.data"); // ignored by git
        let db_file = db_file.to_str().unwrap();
        let mut db_tmp = store::DB::new();

        // test empty
        save_to_file(db_file, &db_tmp);
        let mut db = store::DB::new();
        load_from_file(db_file, &mut db);
        assert_eq!(store::scan("", &db), Vec::<String>::new());

        // test one item
        store::put("foo", "bar".as_bytes(), &mut db_tmp).unwrap();
        save_to_file(db_file, &db_tmp);
        let mut db = store::DB::new();
        load_from_file(db_file, &mut db);
        let v = store::scan("", &db);
        assert_eq!(v.len(), 1);
        assert_eq!(store::get("foo", &db), Some("bar".to_string()));

        // test more items
        store::put("location", "地铁西小口128号".as_bytes(), &mut db_tmp).unwrap();
        store::put("age", "18".as_bytes(), &mut db_tmp).unwrap();
        save_to_file(db_file, &db_tmp);
        let mut db = store::DB::new();
        load_from_file(db_file, &mut db);
        let v = store::scan("", &db);
        assert_eq!(v.len(), 3);
        assert_eq!(store::get("foo", &db), Some("bar".to_string()));
        assert_eq!(store::get("location", &db), Some("地铁西小口128号".to_string()));
        assert_eq!(store::get("age", &db), Some("18".to_string()));

        // test delete item
        store::delete("age", &mut db_tmp).unwrap();
        save_to_file(db_file, &db_tmp);
        let mut db = store::DB::new();
        load_from_file(db_file, &mut db);
        let v = store::scan("", &db);
        assert_eq!(v.len(), 2);
        assert_eq!(store::get("foo", &db), Some("bar".to_string()));
        assert_eq!(store::get("location", &db), Some("地铁西小口128号".to_string()));
    }
}
