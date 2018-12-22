use std::str;
use std::sync::{Arc, Mutex};

use crate::tools;
use crate::store;

pub fn handle_input(cmd: &[u8], arc_db: Arc<Mutex<store::DB>>) -> Result<String, &'static str> {
    let mut db = arc_db.lock().unwrap();

    let _cmd = str::from_utf8(cmd).unwrap().trim();
    if tools::re_contains(r"^ *get  *[^\s]+ *$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Some(x) = store::get(tokens[1], &db) {
            return Ok(x);
        } else {
            return Ok("(None)".to_string());
        }
    }

    if tools::re_contains(r"^ *(put|set)  *[^\s]+  *[^\s]+ *$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Ok(_) = store::put(tokens[1], tokens[2].as_bytes(), &mut db) {
            return Ok("1".to_string());
        } else {
            return Ok("0".to_string());
        }
    }

    if tools::re_contains(r"^ *del  *[^\s]+ *$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Some(x) = store::delete(tokens[1], &mut db) {
            return Ok(x.to_string());
        } else {
            return Ok("(None)".to_string());
        }
    }

    if tools::re_contains(r"^ *scan  *[^\s]+ *$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        let keys = store::scan(&tokens[1], &db);
        let mut result: Vec<u8> = Vec::new();
        for k in keys {
            result.push(0x0C_u8);  // header
            result.push(0x0C_u8);  // header
            let length_bytes = tools::u16_to_bytes(k.len() as u16);
            result.extend(&length_bytes);
            result.extend(k.as_bytes());
        }
        let s = str::from_utf8(&result).unwrap();
        return Ok(s.to_string());
    }

    return Err("Invalid/Unknown Command");
}
