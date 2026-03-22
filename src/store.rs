use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn store (db: &Arc<Mutex<HashMap<String, String>>>, command: Command) -> String {
    match command {
        Command::Get { key } => get(db, key),
        Command::Set {key, value} => set(db, key, value),
        Command::Display => display(db),
        Command::Unknown => String::from("Unknown command")
    }
}

fn set(db: &Arc<Mutex<HashMap<String, String>>>, key: String, value: String) -> String {
    db.lock().unwrap().insert(key, value);
    String::from("OK\n")
}

fn get(db: &Arc<Mutex<HashMap<String, String>>>, key: String) -> String {
    match db.lock().unwrap().get(&key) {
        Some(value) => format!("{}\n", value),
        None => String::from("NOT FOUND\n")
    }
}

fn display(db: &Arc<Mutex<HashMap<String, String>>>) -> String {
    format!("{:?}\n", db.lock().unwrap())
}

pub enum Command {
    Get { key: String },
    Set { key: String, value: String},
    Display,
    Unknown
}