extern crate regex;

mod engine;
mod server;
mod store;
mod tools;

fn main() {
    let mut db = store::DB::new();
    db.insert("foo".to_string(), "bar".to_string());
    server::run(&mut db);
}
