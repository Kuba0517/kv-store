use crate::persistence::KeyDirRecord;
use crate::store::Command;
use crate::store::store;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

pub async fn server(path: PathBuf, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>) {
    const IP_PORT: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(IP_PORT).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db_clone = Arc::clone(db);
        let path_clone = path.clone();
        tokio::spawn(async move {
            println!("Connected!!");
            handle_connection(socket, &path_clone, &db_clone).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, path: &Path, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>) {
    let (reader, writer) = socket.split();
    let mut buf_reader = BufReader::new(reader);
    let mut buf_writer = BufWriter::new(writer);

    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line).await {
            Ok(0) | Err(_) => {
                println!("Client disconnected");
                return;
            }
            Ok(_) => {}
        }

        let response = store(path, db, parse_request(&line.trim()));
        buf_writer.write_all(response.as_bytes()).await.unwrap();
        buf_writer.flush().await.unwrap();
    }
}

fn parse_request(request: &str) -> Command {
    let splitted: Vec<&str> = request.splitn(3, ' ').collect();

    match splitted.as_slice() {
        ["GET", key] => Command::Get {
            key: (*key).to_string(),
        },
        ["SET", key, value] => Command::Set {
            key: (*key).to_string(),
            value: (*value).to_string(),
        },
        ["DELETE", key] => Command::Delete {
            key: (*key).to_string(),
        },
        _ => Command::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_get() {
        let result = parse_request("GET foo");
        assert_eq!(result, Command::Get {key: "foo".to_string()});
    }

    #[test]
    fn parses_delete() {
        let result = parse_request("DELETE foo");
        assert_eq!(result, Command::Delete {key: "foo".to_string()})
    }

    #[test]
    fn parses_set_with_single_value_input() {
        let result = parse_request("SET foo bar1");
        assert_eq!(result, Command::Set {key: "foo".to_string(), value: "bar1".to_string()})
    }

    #[test]
    fn parses_set_with_multi_value_input() {
        let result = parse_request("SET foo bar1 bar2");
        assert_eq!(result, Command::Set { key: "foo".to_string(), value: "bar1 bar2".to_string() });
    }

    #[test]
    fn returns_unknown_command_on_missing_value_for_set() {
        let result = parse_request("SET foo");
        assert_eq!(result, Command::Unknown);
    }

    #[test]
    fn returns_unknown_command_one_random_command() {
        let result = parse_request("RANDOM foo");
        assert_eq!(result, Command::Unknown)
    }
}