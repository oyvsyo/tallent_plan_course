// #![deny(missing_docs)]
//! Module with key-value storage
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::engine::KvsEngine;
use crate::error::{KVSError, Result};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;
const DATABASE_FILENAME: &str = "kvs.db";

// TODO: its duplicated in kvs.rs for cli usage
#[derive(Serialize, Deserialize)]
enum DBInsertion {
    /// Set up value by key into KVS
    Set { key: String, value: String },
    /// Removes value by key
    Rm { key: String },
}

/// Database insertion log
impl DBInsertion {
    pub fn get_key(&self) -> &String {
        match self {
            DBInsertion::Rm { key } => key,
            DBInsertion::Set { key, .. } => key,
        }
    }
}

#[derive(Debug, Clone)]
struct ItemPosition {
    pos: u64,
    len: usize,
}

/// Usage
/// ```rust
/// # use std::error::Error;
/// # use assert_cmd::prelude::*;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use std::path::Path;
/// use kvs::KvStore;
///
/// let mut path = Path::new(".");
///
/// let mut store = KvStore::open(path).unwrap();
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
    storage: HashMap<String, ItemPosition>,
    possible_compaction: u64,
    file: File,
}

impl KvsEngine for KvStore {
    /// Set up value by key into KVS
    fn set(&mut self, key: String, value: String) -> Result<()> {
        if self.possible_compaction > COMPACTION_THRESHOLD {
            let _ = self.compaction();
        }

        let insertion = DBInsertion::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let insertion_str = serde_json::to_string(&insertion)?;
        let len = insertion_str.len();
        // move to end of the file and then write
        let pos = self.file.seek(SeekFrom::End(0 as i64))?;

        let index = ItemPosition { pos, len };

        self.file.write_all(insertion_str.as_bytes())?;
        self.file.flush()?;
        if let Some(_old_position) = self.storage.insert(key.clone(), index) {
            self.possible_compaction += len as u64;
        }
        Ok(())
    }
    /// Get value by key
    fn get(&mut self, key: String) -> Result<Option<String>> {
        // println!("{:?}", self.storage);
        let record_option = self.storage.get(key.as_str());
        match record_option {
            Some(record) => {
                let mut buf_reader = BufReader::new(&self.file);
                buf_reader.seek(SeekFrom::Start(record.pos))?;
                let handle = buf_reader.take(record.len as u64);

                let insertion: DBInsertion = serde_json::from_reader(handle)?;
                match insertion {
                    DBInsertion::Set { value, .. } => Ok(Some(value)),
                    _ => Err(KVSError::GeneralKVSError),
                }
            }
            None => Ok(None),
        }
    }
    /// Removes value by key
    fn remove(&mut self, key: String) -> Result<()> {
        let insertion = DBInsertion::Rm { key: key.clone() };

        if let Some(old_insertion_pos) = self.storage.remove(key.as_str()) {
            // move to end of the file and then write
            let insertion_str = serde_json::to_string(&insertion)?;
            self.file.seek(SeekFrom::End(0 as i64))?;
            self.file.write_all(insertion_str.as_bytes())?;
            self.file.flush()?;
            self.possible_compaction += old_insertion_pos.len as u64;
            Ok(())
        } else {
            Err(KVSError::GeneralKVSError)
        }
    }
}

impl KvStore {
    /// Create new instance
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(true)
            .open(path)
            .unwrap();
        let possible_compaction = 0;
        let storage = HashMap::new();

        let mut obj = Self {
            storage,
            possible_compaction,
            file,
        };
        obj.create_index()?;
        Ok(obj)
    }

    fn create_index(&mut self) -> Result<()> {
        let buf_reader = BufReader::new(&self.file);

        let mut stream = Deserializer::from_reader(buf_reader).into_iter::<DBInsertion>();
        let mut start = 0;
        // loop over all commands deserialized in file
        while let Some(Ok(insertion)) = stream.next() {
            let end = stream.byte_offset();
            let len = end - start;

            let position = ItemPosition {
                pos: start as u64,
                len,
            };
            start = end;
            // insert or remove keys from memory
            // sum up repeated keys for compaction acountability
            match insertion {
                DBInsertion::Set { key, .. } => {
                    if let Some(old_insertion_pos) = self.storage.insert(key.to_owned(), position) {
                        self.possible_compaction += old_insertion_pos.len as u64;
                    }
                }
                DBInsertion::Rm { key } => {
                    if let Some(old_insertion_pos) = self.storage.remove(key.as_str()) {
                        self.possible_compaction += old_insertion_pos.len as u64;
                    }
                }
            }
        }
        Ok(())
    }

    fn compaction(&mut self) -> Result<()> {
        log::info!("Compaction triggered");
        self.file.seek(SeekFrom::Start(0 as u64))?;
        let buf_reader = BufReader::new(&self.file);

        let mut stream = Deserializer::from_reader(buf_reader).into_iter::<DBInsertion>();

        let mut index: HashMap<String, String> = HashMap::new();
        while let Some(Ok(insertion)) = stream.next() {
            self.storage.remove(insertion.get_key());
            match insertion {
                DBInsertion::Set { key, value } => {
                    index.insert(key.to_owned(), value.to_owned());
                }
                DBInsertion::Rm { key } => {
                    index.remove(key.as_str());
                }
            }
        }
        // flush file
        self.file.set_len(0)?;
        self.possible_compaction = 0;
        for (key, value) in index {
            self.set(key, value)?
        }
        Ok(())
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path_buf = path.into();
        path_buf.push(DATABASE_FILENAME);
        KvStore::new(path_buf)
    }
}
