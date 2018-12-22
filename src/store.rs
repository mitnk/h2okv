/// Here we should use some DS like red-black tree. For simple, I may
/// implement a `2–3 tree`[0] in later commits soon. For now, let's start
/// with Rust's builtin: `std::collections::HashMap`.
///
/// [0] https://en.wikipedia.org/wiki/2-3_tree
use std::collections::HashMap;
use std::str;

use crate::persistence;

// FIXME: we should use `HashMap<&[u8], &[u8]>` here,
// using String for it to work for now.
pub type DB = HashMap<String, String>;

pub fn get(key: &str, db: &DB) -> Option<String> {
    match db.get(key) {
        Some(x) => {
            Some(x.clone())
        }
        None => None
    }
}

pub fn put(key: &str, value: &[u8], db: &mut DB) -> Result<(), &'static str> {
    let data = str::from_utf8(value).unwrap();
    db.insert(key.to_string(), data.to_string());
    persistence::save_to_file(db);
    Ok(())
}
