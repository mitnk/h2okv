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

pub fn delete(key: &str, db: &mut DB) -> Option<String> {
    match db.remove(key) {
        Some(x) => {
            Some(x)
        }
        None => None
    }
}

pub fn scan(key: &str, db: &DB) -> Vec<String> {
    let mut result = Vec::new();
    for k in db.keys() {
        if k.contains(key) {
            result.push(k.to_string());
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::{DB, delete, get, put, scan};

    #[test]
    fn test_store() {
        let mut db = DB::new();
        assert_eq!(get("foo", &db), None);

        assert!(put("foo", "bar".as_bytes(), &mut db).is_ok());
        assert_eq!(get("foo", &db), Some("bar".to_string()));
        assert_eq!(scan("f", &db), vec!["foo".to_string()]);
        assert_eq!(
            scan("z", &db),
            Vec::<String>::new(),
        );

        assert!(put("find", "rust".as_bytes(), &mut db).is_ok());
        let v = scan("f", &db);
        assert!(v.contains(&String::from("find")));
        assert!(v.contains(&String::from("foo")));

        assert_eq!(delete("foo", &mut db), Some("bar".to_string()));
        assert_eq!(get("foo", &db), None);
        assert_eq!(scan("f", &db), vec!["find".to_string()]);

        assert_eq!(delete("foo", &mut db), None);
    }
}
