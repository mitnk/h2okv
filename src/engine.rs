use std::str;

use crate::tools;
use crate::store;

pub fn handle_input(cmd: &[u8], db: &mut store::DB) -> Result<String, &'static str> {
    let _cmd = str::from_utf8(cmd).unwrap().trim();
    if tools::re_contains(r"^ *get  *\w+$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Some(x) = store::get(tokens[1], db) {
            return Ok(x);
        } else {
            return Ok("(None)".to_string());
        }
    } else {
        println!("re not match: {:?}", _cmd);
    }


    if tools::re_contains(r"^ *put  *\w+  *\w+$", _cmd) {
        let tokens: Vec<&str> = _cmd.split_whitespace().collect();
        if let Ok(_) = store::put(tokens[1], tokens[2].as_bytes(), db) {
            return Ok("true".to_string());
        } else {
            return Ok("false".to_string());
        }
    }
    if cmd.starts_with(b"del ") {
        return Ok("del: todo".to_string());
    }
    if cmd.starts_with(b"scan ") {
        return Ok("scan: todo".to_string());
    }
    return Err("Invalid/Unknown Command");
}
