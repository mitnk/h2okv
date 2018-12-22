extern crate byteorder;
extern crate regex;

mod engine;
mod persistence;
mod server;
mod store;
mod tools;

fn main() {
    let mut db = store::DB::new();
    server::run(&mut db);
}
