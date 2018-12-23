use std::net::TcpStream;

use crate::do_get;
use crate::do_scan;

pub fn query(line: &str, stream: &mut TcpStream) {
    if line.starts_with("get ") {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        do_get::get(tokens[1], stream);
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
}
