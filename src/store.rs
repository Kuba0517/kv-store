use crate::persistence::{read_value, save, KeyDirRecord, StoreRecord};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn store(db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, command: Command) -> String {
    match command {
        Command::Get { key } => get(db, &key),
        Command::Set { key, value } => set(db, &key, &value),
        Command::Delete { key } => delete(db, &key),
        // Command::Display => display(db),
        Command::Unknown => String::from("Unknown command\n"),
    }
}

fn set(db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, key: &str, value: &str) -> String {
    let db_key = String::from(key);
    db.lock().unwrap().insert(db_key, save(key, value));
    String::from("OK\n")
}

fn get(db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, key: &str) -> String {
    match db.lock().unwrap().get(key) {
        Some(value) => read_value(value),
        None => String::from("NOT FOUND\n"),
    }
}

fn delete(db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>, key: &str) -> String {
    match db.lock().unwrap().remove(key) {
        Some(_) => {
            // save(db);
            String::from("OK\n")
        },
        None => String::from("NOT FOUND\n")
    }
}

fn display(db: &Arc<Mutex<HashMap<String, String>>>) -> String {
    format!("{:?}\n", db.lock().unwrap())
}

pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    // Display,
    Unknown,
}
