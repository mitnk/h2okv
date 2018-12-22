extern crate byteorder;
extern crate regex;

use std::sync::{Arc, Mutex};

mod engine;
mod persistence;
mod server;
mod store;
mod tools;

fn main() {
    let arc_db = Arc::new(Mutex::new(store::DB::new()));
    persistence::load_from_file(arc_db.clone());
    server::run(arc_db.clone());
}
