use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::persistence::{load, save};

mod server;
mod store;
mod persistence;

fn main() {
    let db: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(load()));
    server::server(&db);
}
