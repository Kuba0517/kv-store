use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::store::store;
use crate::store::Command;

pub fn server(db: &Arc<Mutex<HashMap<String, String>>>) {
    const IP_PORT: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(IP_PORT).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connected!!");
        handle_connection(stream, db);
    }
}

fn handle_connection(stream: TcpStream, db: &Arc<Mutex<HashMap<String, String>>>) {
    let reader_stream = stream.try_clone().unwrap();
    let mut reader = BufReader::new(reader_stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let response = store(db, parse_request(line));
        writer.write_all(response.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}

fn parse_request(request: String) -> Command {
    let splitted: Vec<&str> = request.split_whitespace().collect();

    match splitted.as_slice() {
        ["GET", key] => Command::Get {
            key: (*key).to_string(),
        },
        ["SET", key, value] => Command::Set {
            key: (*key).to_string(),
            value: (*value).to_string(),
        },
        ["DISPLAY"] => Command::Display,
        _ => Command::Unknown,
    }
}