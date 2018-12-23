use std::net::TcpStream;

use crate::do_delete;
use crate::do_get;
use crate::do_put;
use crate::do_scan;

pub fn query(line: &str, stream: &mut TcpStream) {
    if line.starts_with("del ") {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        do_delete::delete(tokens[1], stream);
        return;
    }

    if line.starts_with("get ") {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        do_get::get(tokens[1], stream);
        return;
    }

    if line.starts_with("put ") || line.starts_with("set ") {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 3 {
            println!("invalid command");
            return;
        }
        do_put::put(tokens[1], tokens[2], stream);
        return;
    }

    if line.starts_with("scan") {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() > 1 {
            do_scan::scan(tokens[1], stream);
        } else {
            do_scan::scan("", stream);
        }
        return;
    }

    if !line.is_empty() {
        println!("unknown command: {:?}", line);
    }
}
