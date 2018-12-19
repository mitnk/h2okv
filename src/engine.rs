use std::str;

use crate::tools;
use crate::store;

pub fn handle_get(key: &str) -> Option<&'static [u8]> {
    store::get(key)
}

pub fn handle_put(key: &str, value: &[u8]) -> Result<(), &'static str> {
    store::put(key, value)
}

pub fn handle_input(cmd: &[u8]) -> Result<&'static [u8], &'static str> {
    let _cmd = str::from_utf8(cmd).unwrap().trim();
    if tools::re_contains(r"^ *get  *\w+$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Some(x) = handle_get(tokens[1]) {
            return Ok(x);
        } else {
            return Ok("(None)".as_bytes());
        }
    } else {
        println!("re not match: {:?}", _cmd);
    }


    if tools::re_contains(r"^ *put  *\w+  *\w+$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Ok(_) = handle_put(tokens[1], tokens[2].as_bytes()) {
            return Ok("true".as_bytes());
        } else {
            return Ok("false".as_bytes());
        }
    }
    if cmd.starts_with(b"del ") {
        return Ok("del: todo".as_bytes());
    }
    if cmd.starts_with(b"scan ") {
        return Ok("scan: todo".as_bytes());
    }
    return Err("Invalid/Unknown Command");
}
