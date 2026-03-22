use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn store (db: &Arc<Mutex<HashMap<String, String>>>, command: Command) {
    match command {
        Command::Get { key } => get(db, key),
        Command::Set {key, value} => set(db, key, value),
        Command::Unknown => println!("wrong!")
    }
}

fn set(db: &Arc<Mutex<HashMap<String, String>>>, key: String, value: String) {
    db.lock().unwrap().insert(key, value);
    println!("The value has been inserted, the current store look {:?}", db.lock().unwrap())
}

fn get(db: &Arc<Mutex<HashMap<String, String>>>, key: String) {
    match db.lock().unwrap().get(&key) {
        Some(value) => println!("{}", value),
        None => println!("NOT FOUND")
    }
}

pub enum Command {
    Get { key: String },
    Set { key: String, value: String},
    Unknown
}