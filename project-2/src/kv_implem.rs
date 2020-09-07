use failure::Fail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

pub trait KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
    fn open() -> Result<KvStorePersist>;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
#[derive(Debug, PartialEq)]
enum KvPersistCommand {
    Set,
    Delete,
}

#[derive(Serialize, Deserialize, Debug)]
struct KvEntry {
    key: String,
    value: Option<String>,
    command: KvPersistCommand,
}

pub struct KvStorePersist {
    db_file: String,
    map_store: HashMap<String, usize>,
}

#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Removing non-existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStorePersist {
    pub fn new(db_file: String) -> Self {
        let kv_store_persist = Self {
            map_store: HashMap::new(),
            db_file: db_file,
        };
        kv_store_persist.create_db_file_if_not_exist();
        return kv_store_persist;
    }

    fn set_in_memory(&mut self, key: String, value: usize) {
        self.map_store.insert(key, value);
    }

    fn rm_in_memory(&mut self, key: String) {
        self.map_store.remove(&key);
    }

    fn load_database_from_logs(&mut self) {
        let file = File::open(self.db_file.clone()).expect("unable to read file content");
        let reader = BufReader::new(file);
        let mut current_position: usize = 0;

        for line in reader.lines() {
            let unwrapped_line = line.unwrap();
            let db_entry: KvEntry = serde_json::from_str(&unwrapped_line).unwrap();
            if db_entry.command == KvPersistCommand::Set {
                self.set_in_memory(db_entry.key, current_position)
            } else {
                self.rm_in_memory(db_entry.key)
            }
            current_position += unwrapped_line.len() + 1;
        }
    }

    fn get_entry_from_pos(&self, pos: usize) -> KvEntry {
        let f1 = File::open(self.db_file.clone()).expect("unable to read file content");
        let mut reader = BufReader::new(f1);
        reader.seek(SeekFrom::Start(pos as u64)).unwrap();
        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .expect("unable to read file content");
        let db_entry: KvEntry = serde_json::from_str(&buf).unwrap();
        return db_entry;
    }

    fn create_db_file_if_not_exist(&self) {
        let file_exist = Path::new(&self.db_file).exists();
        if !file_exist {
            File::create(self.db_file.clone()).expect("unable to create db file");
        }
    }

    fn save_entry_in_logs(&self, entry: KvEntry) {
        let serialized = serde_json::to_string(&entry).unwrap() + "\n";
        let mut file_open_options = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.db_file.clone())
            .expect("unable to open db file");
        file_open_options
            .write_all(serialized.as_bytes())
            .expect("unable to write into db file");
    }
}

impl KvStore for KvStorePersist {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let option_value: Option<String> = Some(value.clone());
        let entry = KvEntry {
            key: key.clone(),
            value: option_value,
            command: KvPersistCommand::Set,
        };
        self.save_entry_in_logs(entry);
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        self.load_database_from_logs();
        let entry = self.get_entry_from_pos(self.map_store.get(&key).cloned().unwrap());
        return Ok(entry.value);
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.load_database_from_logs();
        let entry = KvEntry {
            key: key.clone(),
            value: None,
            command: KvPersistCommand::Delete,
        };
        self.save_entry_in_logs(entry);
        self.rm_in_memory(key);
        Ok(())
    }

    fn open() -> Result<KvStorePersist> {
        // let mut rng = rand::thread_rng();
        return Ok(KvStorePersist::new(String::from("db.db")));
    }
}
