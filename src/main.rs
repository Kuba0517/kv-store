use crate::persistence::{KeyDirRecord, load};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod persistence;
mod server;
mod store;

#[tokio::main]
async fn main() {
    let db: Arc<Mutex<HashMap<String, KeyDirRecord>>> = Arc::new(Mutex::new(load()));
    server::server(&db).await;
}
