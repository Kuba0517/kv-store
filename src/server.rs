use crate::persistence::KeyDirRecord;
use crate::store::Command;
use crate::store::store;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

pub async fn server(db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>) {
    const IP_PORT: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(IP_PORT).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db_clone = Arc::clone(db);
        tokio::spawn(async move {
            println!("Connected!!");
            handle_connection(socket, &db_clone).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, db: &Arc<Mutex<HashMap<String, KeyDirRecord>>>) {
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

        let response = store(db, parse_request(&line.trim()));
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
