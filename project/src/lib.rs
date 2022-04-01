// #![deny(missing_docs)]
//! Module with key-value storage
use std::collections::HashMap;
use std::path::{PathBuf,Path};
use std::ops::Add;
use std::fs::{write, OpenOptions, File};
use std::io::{self, BufRead, Write, SeekFrom};
use std::io::BufReader;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};


/// Usage
/// ```rust
/// # use std::error::Error;
/// # use assert_cmd::prelude::*;
/// # use std::path::PathBuf;
/// # fn main() -> Result<(), Box<dyn Error>> {
///
/// use kvs::KvStore;
///
/// let mut path_buf = PathBuf::from(".");
/// path_buf.push("file.bk");
///
/// let mut store = KvStore::new(path_buf).unwrap();
/// store.set("key1".to_owned(), "value1".to_owned());
/// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
/// store.remove("key1".to_owned());
/// #
/// #     Ok(())
/// # }
/// ```

/// In memory key value storage String:String
///

#[derive(Debug)]
pub enum KVSError {
    GeneralKVSError
}

impl fmt::Display for KVSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No matching cities with a population were found.")
    }
}

impl Error for KVSError {
    fn description(&self) -> &str {
        "Fuck"
    }
}

pub type KVSResult<T> = Result<T, KVSError>;

// TODO: its duplicated in kvs.rs for cli usage
#[derive(Serialize, Deserialize)]
enum KVSCommands {
    /// Set up value by key into KVS
    Set { key: String, value: String },
    /// Get value by key
    Get { key: String },
    /// Removes value by key
    Rm { key: String },
}


#[derive(Debug)]
pub struct KvStore {
    storage: HashMap<String, String>,
    len: usize,
    file: File
}

impl KvStore {
    /// Create new instance
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(true)
            .open(path)
            .unwrap();
        let file_length = 0; //file.lines().count();
        let key_line_map = HashMap::new();

        let mut obj = Self {
            storage: key_line_map,
            len: file_length,
            file
        };
        obj.create_index();
        Ok(obj)
    }

    fn create_index(&mut self) -> Result<(), String> {
        let buf_reader = BufReader::new(&self.file);
        for (i, line) in buf_reader.lines().enumerate() {
            let line_str: String = line.expect("cant read");
            let cmd: KVSCommands = serde_json::from_str(
                line_str.as_str()).expect("cant parse"
            );
            match cmd {
                KVSCommands::Set { key, value } => {
                    self.storage.insert(key.to_owned(), value.to_owned());
                }
                KVSCommands::Rm { key} => {
                    self.storage.remove(key.as_str());
                }
                _ => ()
            }
            self.len = i
        }
        Ok(())
    }

    /// Set up value by key into KVS
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let cmd = KVSCommands::Set {
            key: key.clone(),
            value: value.clone()
        };
        let cmd_str = serde_json::to_string(&cmd)
            .expect("Cant serialize ((");
        &self.file.write_all(cmd_str.add("\n").as_bytes())
            .expect("Cant write to file");
        self.storage.insert(key.clone(), value.clone());
        Ok(())
    }
    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>, String> {
        let maybe_index = self.storage.get(key.as_str()).cloned();
        Ok(maybe_index)
    }
    /// Removes value by key
    pub fn remove(&mut self, key: String) -> Result<(), KVSError> {
        let cmd = KVSCommands::Rm { key: key.clone() };
        let cmd_str = serde_json::to_string(&cmd)
            .expect("Cant serialize ((");
        &self.file.write_all(cmd_str.add("\n").as_bytes())
            .expect("Cant write to file");
        match self.storage.remove(key.as_str()) {
            Some(v) => Ok(()),
            None => Err(KVSError::GeneralKVSError)
        }
    }
    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: &Path) -> Result<KvStore, String> {
        let mut path_buf = path.to_path_buf();
        path_buf.push("file.bk");
        KvStore::new(path_buf)
    }
}
