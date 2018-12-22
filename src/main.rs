extern crate byteorder;
extern crate regex;

mod engine;
mod persistence;
mod server;
mod store;
mod tools;

fn main() {
    let mut db = store::DB::new();
    persistence::load_from_file(&mut db);
    server::run(&mut db);
}
