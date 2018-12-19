/// Here we should use some DS like red-black tree. For simple, I may
/// implement a `2–3 tree`[0] in later commits soon. For now, let's start
/// with Rust's builtin: `std::collections::HashMap`.
///
/// [0] https://en.wikipedia.org/wiki/2-3_tree
use std::collections::HashMap;

pub type _DB = HashMap<&'static str, &'static [u8]>;

pub fn get(_key: &str) -> Option<&'static [u8]> {
    None
}

pub fn put(_key: &str, _value: &[u8]) -> Result<(), &'static str> {
    Err("todo")
}
