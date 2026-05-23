use crate::persistence::{KeyDirRecord, read_value, save};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;

pub fn store(path: &Path, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, command: Command) -> String {
    match command {
        Command::Get { key } => get(path, db, &key),
        Command::Set { key, value } => set(path, db, &key, &value),
        Command::Delete { key } => delete(path, db, &key),
        Command::Unknown => String::from("Unknown command\n"),
    }
}

fn set(path: &Path, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, key: &str, value: &str) -> String {
    let db_key = String::from(key);
    db.lock().unwrap().insert(db_key, save(path, key, value));
    String::from("OK\n")
}

fn get(path: &Path, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, key: &str) -> String {
    match db.lock().unwrap().get(key) {
        Some(value) => read_value(path, value),
        None => String::from("NOT FOUND\n"),
    }
}

fn delete(path: &Path, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, key: &str) -> String {
    match db.lock().unwrap().remove(key) {
        Some(_) => {
            save(path, key, "");
            String::from("OK\n")
        }
        None => String::from("NOT FOUND\n"),
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Unknown,
}
