// #![deny(missing_docs)]
//! Module with key-value storage
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::error::{KVResult, KVSError};

// TODO: its duplicated in kvs.rs for cli usage
#[derive(Serialize, Deserialize)]
enum KVSCommands {
    /// Set up value by key into KVS
    Set { key: String, value: String },
    /// Removes value by key
    Rm { key: String },
}

#[derive(Debug, Clone)]
struct KVSPosition {
    position: u64,
    len: usize,
}

/// Usage
/// ```rust
/// # use std::error::Error;
/// # use assert_cmd::prelude::*;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use std::path::PathBuf;
/// use kvs::KvStore;
///
/// let mut path_buf = PathBuf::from(".");
/// path_buf.push("file.bk");
///
/// let mut store = KvStore::new(path_buf).unwrap();
/// store.set("key1".to_owned(), "value1".to_owned());
/// assert_eq!(store.get("key1".to_owned())?, Some("value1".to_owned()));
/// store.remove("key1".to_owned());
/// #
/// #     Ok(())
/// # }
/// ```

/// In memory key value storage String:String
///
#[derive(Debug)]
pub struct KvStore {
    storage: HashMap<String, KVSPosition>,
    file_len: usize,
    file: File,
}

impl KvStore {
    /// Create new instance
    pub fn new(path: PathBuf) -> KVResult<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(true)
            .open(path)
            .unwrap();
        let file_len = 0;
        let storage = HashMap::new();

        let mut obj = Self {
            storage,
            file_len,
            file,
        };
        obj.create_index()?;
        Ok(obj)
    }

    fn create_index(&mut self) -> Result<(), KVSError> {
        let buf_reader = BufReader::new(&self.file);

        let mut stream = Deserializer::from_reader(buf_reader).into_iter::<KVSCommands>();
        let mut start = 0;
        while let Some(Ok(cmd)) = stream.next() {
            let end = stream.byte_offset();
            let len = end - start;
            // println!("start, end: {} {}", start, end);
            let index = KVSPosition {
                position: start as u64,
                len: len as usize,
            };
            // println!("{:?}", index);
            start = end;
            match cmd {
                KVSCommands::Set { key, ..} => {
                    self.storage.insert(key.to_owned(), index);
                    self.file_len += 1;
                }
                KVSCommands::Rm { key } => {
                    self.storage.remove(key.as_str());
                    self.file_len += 1;
                }
            }
        }
        Ok(())
    }

    fn compaction(&mut self) -> KVResult<()> {
        // println!("Compaction triggered");
        let _ = &self.file.seek(SeekFrom::Start(0 as u64));
        let buf_reader = BufReader::new(&self.file);

        let mut stream = Deserializer::from_reader(buf_reader).into_iter::<KVSCommands>();

        let mut index: HashMap<String, String> = HashMap::new();
        while let Some(Ok(cmd)) = stream.next() {
            match cmd {
                KVSCommands::Set { key, value } => {
                    index.insert(key.to_owned(), value.to_owned());
                    self.storage.remove(key.as_str());
                }
                KVSCommands::Rm { key } => {
                    index.remove(key.as_str());
                    self.storage.remove(key.as_str());
                }
            }
        }
        // flush file
        self.file.set_len(0)?;
        self.file_len = 0;
        // println!("{:?}", index);

        for (key, value) in index {
            self.set(key, value)?
        }
        Ok(())
    }

    /// Set up value by key into KVS
    pub fn set(&mut self, key: String, value: String) -> KVResult<()> {
        if self.file_len - self.storage.len() > 2000 {
            // println!("compaction triggered {}", self.storage.len());
            let _ = self.compaction();
        }

        let cmd = KVSCommands::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let cmd_str = serde_json::to_string(&cmd)?;
        let len = cmd_str.len();
        // move to end of the file and then write
        let position = self.file.seek(SeekFrom::End(0 as i64))?;
        
        let index = KVSPosition {
            position,
            len
        };

        self.file.write_all(cmd_str.as_bytes())?;
        self.storage.insert(key.clone(), index);
        self.file_len += 1;
        Ok(())
    }
    /// Get value by key
    pub fn get(&self, key: String) -> KVResult<Option<String>> {
        // println!("{:?}", self.storage);
        let record_option = self.storage.get(key.as_str());
        match record_option {
            Some(record) => {
                let mut buf_reader = BufReader::new(&self.file);
                buf_reader.seek(SeekFrom::Start(record.position))?;
                let mut handle = buf_reader.take(record.len as u64);

                let mut buf = String::new();
                handle.read_to_string(&mut buf)?;

                let cmd: KVSCommands = serde_json::from_str(buf.as_str())?;
                match cmd {
                    KVSCommands::Set { value, .. } => Ok(Some(value)),
                    _ => Err(KVSError::GeneralKVSError),
                }
            }
            None => Ok(None),
        }
    }
    /// Removes value by key
    pub fn remove(&mut self, key: String) -> KVResult<()> {
        let cmd = KVSCommands::Rm { key: key.clone() };
        let cmd_str = serde_json::to_string(&cmd)?;

        // move to end of the file and then write
        self.file.seek(SeekFrom::End(0 as i64))?;
        self.file.write_all(cmd_str.as_bytes())?;
        self.file_len += 1;
        match self.storage.remove(key.as_str()) {
            Some(_) => Ok(()),
            None => Err(KVSError::GeneralKVSError),
        }
    }
    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: &Path) -> KVResult<KvStore> {
        let mut path_buf = path.to_path_buf();
        path_buf.push("file.bk");
        KvStore::new(path_buf)
    }
}
