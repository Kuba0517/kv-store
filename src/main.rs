use crate::persistence::{KeyDirRecord, load};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod persistence;
mod server;
mod store;

#[tokio::main]
async fn main() {
    let path = PathBuf::from("./db.bin");
    let db: Arc<Mutex<HashMap<String, KeyDirRecord>>> = Arc::new(Mutex::new(load(&path)));
    server::server(path, &db).await;
}
