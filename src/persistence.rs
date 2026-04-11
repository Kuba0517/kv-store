use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const FILE_PATH: &str = "./db.bin";
const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_CKSUM);

pub struct StoreRecord {
    crc: u32,
    timestamp: u64,
    key_size: u32,
    value_size: u32,
    key: Vec<u8>,
    value: Vec<u8>
}

pub struct KeyDirRecord {
    file_id: u32,
    value_size: u32,
    value_pos: usize,
    timestamp: u64
}

pub fn load() -> HashMap<String, KeyDirRecord> {
    let mut db = HashMap::new();
    let file = match File::open(FILE_PATH) {
        Ok(f) => f,
        Err(_) => return db,
    };

    let mut buff = BufReader::new(file);

    let mut buff_crc: [u8; 4] = [0; 4];
    let mut buff_timestamp: [u8; 8] = [0; 8];
    let mut buff_key_size: [u8; 4] = [0; 4];
    let mut buff_value_size: [u8; 4] = [0; 4];

    let mut timestamp_value: u64;
    let mut key_size_value: u32;
    let mut value_size_value: u32;
    let mut key: String;

    loop {
        if buff.read_exact(&mut buff_crc).is_err() {
            break;
        }
        buff.read_exact(&mut buff_timestamp).unwrap();
        buff.read_exact(&mut buff_key_size).unwrap();
        buff.read_exact(&mut buff_value_size).unwrap();

        timestamp_value = u64::from_le_bytes(buff_timestamp);
        key_size_value = u32::from_le_bytes(buff_key_size);
        value_size_value = u32::from_le_bytes(buff_value_size);

        let mut buff_key: Vec<u8> = vec![0; key_size_value as usize];

        buff.read_exact(&mut buff_key[..]).unwrap();

        key = String::from_utf8(buff_key).unwrap();

        let value_pos = buff.stream_position().unwrap();

        buff.seek(SeekFrom::Current(value_size_value as i64)).unwrap();

        db.insert(key, KeyDirRecord {
            file_id: 1,
            value_size: value_size_value,
            value_pos: value_pos as usize,
            timestamp: timestamp_value
        });
    }

    db
}

pub fn save(key: &str, value: &str) -> KeyDirRecord {
    let file_path = Path::new(FILE_PATH);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let key_size = key.as_bytes().len() as u32;
    let value_size = value.as_bytes().len() as u32;

    let mut checksum_data = Vec::new();
    checksum_data.extend(timestamp.to_le_bytes());
    checksum_data.extend(key_size.to_le_bytes());
    checksum_data.extend(value_size.to_le_bytes());
    checksum_data.extend(key.as_bytes());
    checksum_data.extend(value.as_bytes());

    let crc = X25.checksum(&checksum_data);

    file.write_all(&crc.to_le_bytes()).expect("Wasn't able to save crc");
    file.write_all(&timestamp.to_le_bytes()).expect("Wasn't able to save timestamp");
    file.write_all(&key_size.to_le_bytes()).expect("Wasn't able to save key_size");
    file.write_all(&value_size.to_le_bytes()).expect("Wasn't able to save value_size");
    file.write_all(key.as_bytes()).expect("Wasn't able to save key");

    let value_pos = file.stream_position().unwrap();

    file.write_all(value.as_bytes()).expect("Wasn't able to save value");

    KeyDirRecord {
        file_id: 1,
        value_size,
        value_pos: value_pos as usize,
        timestamp
    }
}

pub fn read_value(record: &KeyDirRecord) -> String {

    let mut file = File::open(FILE_PATH).unwrap();

    file.seek(SeekFrom::Start(record.value_pos as u64)).unwrap();

    let mut buff_value: Vec<u8> = vec![0; record.value_size as usize];

    file.read_exact(&mut buff_value[..]).unwrap();

    format!("{}\n", String::from_utf8(buff_value).unwrap())
}