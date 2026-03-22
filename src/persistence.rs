use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::io::{Write};

const FILE_PATH: &str = "./db.txt";

pub fn load() -> HashMap<String, String> {
    let mut db = HashMap::new();

    let contents = fs::read_to_string(FILE_PATH).expect("Should be able to read db file");

    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let kv_vec: Vec<&str> = line.split(":").collect();
        db.insert(kv_vec[0].to_string(), kv_vec[1].to_string());
    }

    db
}

pub fn save(db: &Arc<Mutex<HashMap<String, String>>>) {
    let mut file = fs::File::create(FILE_PATH).unwrap();

    for (key, value) in db.lock().unwrap().iter() {
        writeln!(file, "{}:{}", key, value).unwrap();
    }
}