use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::persistence::{load, KeyDirRecord};

mod server;
mod store;
mod persistence;

#[tokio::main]
async fn main() {
    let db: Arc<Mutex<HashMap<String, KeyDirRecord>>> = Arc::new(Mutex::new(load()));
    server::server(&db).await;
}
