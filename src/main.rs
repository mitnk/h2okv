extern crate byteorder;

use std::sync::{Arc, Mutex};

mod persistence;
mod server;
mod store;
mod tools;

fn main() {
    let arc_db = Arc::new(Mutex::new(store::DB::new()));
    load_from_file_arc(arc_db.clone());
    server::run(arc_db.clone());
}

fn load_from_file_arc(arc_db: Arc<Mutex<store::DB>>) {
    let clone_arc = arc_db.clone();
    let mut db = clone_arc.lock().unwrap();
    if let Some(db_file) = tools::get_db_file() {
        persistence::load_from_file(&db_file, &mut db);
    }
}
