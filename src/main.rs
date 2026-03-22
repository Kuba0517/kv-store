use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

mod server;
mod store;


fn main() {
    let db: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    server::server(&db)
}
