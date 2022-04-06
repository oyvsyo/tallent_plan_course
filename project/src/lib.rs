// #![deny(missing_docs)]
//! Module with key-value storage
use std::collections::HashMap;
use std::path::{PathBuf,Path};
use std::fs::{OpenOptions, File};
use std::io::{Write, Read, BufRead, BufWriter, BufReader, Seek, SeekFrom};
use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

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
        "KVS error .. please get some help"
    }
}

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

#[derive(Debug,Clone)]
struct KVSPosition {
    position: u64,
    len: usize
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
        let file_length = 0;
        let key_line_map = HashMap::new();

        let mut obj = Self {
            storage: key_line_map,
            file_len: file_length,
            file
        };
        obj.create_index().expect("Cant create index");
        Ok(obj)
    }

    fn create_index(&mut self) -> Result<(), KVSError> {
        let buf_reader = BufReader::new(&self.file);

        let mut stream = Deserializer::from_reader(buf_reader).into_iter::<KVSCommands>();
        let mut start = 0;
        while let Some(Ok(cmd)) = stream.next() {
            let end = stream.byte_offset();
            let len =  end - start;
            // println!("start, end: {} {}", start, end);
            let index = KVSPosition{
                position: start as u64,
                len: len as usize
            };
            // println!("{:?}", index);
            start = end;
            match cmd {
                KVSCommands::Set { key, value: _ } => {
                    self.storage.insert(key.to_owned(), index);
                    self.file_len += 1;
                }
                KVSCommands::Rm { key } => {
                    self.storage.remove(key.as_str());
                    self.file_len += 1;
                }
                _ => ()
            }
        }
        Ok(())
    }

    fn compaction(&mut self) -> Result<(), std::io::Error> {
        // println!("Compaction triggered");
        let _ = &self.file.seek(SeekFrom::Start(0 as u64));
        let buf_reader = BufReader::new(&self.file);

        let mut stream = Deserializer::from_reader(buf_reader).into_iter::<KVSCommands>();
        
        let mut index: HashMap<String, String> = HashMap::new();
        while let Some(Ok(cmd)) = stream.next(){
            match cmd {
                KVSCommands::Set { key, value } => {
                    index.insert(key.to_owned(), value.to_owned());
                    self.storage.remove(key.as_str());
                }
                KVSCommands::Rm { key } => {
                    self.storage.remove(key.as_str());
                    index.remove(key.as_str());
                }
                _ => ()
            }
        }
        // flush file 
        self.file.set_len(0)?;
        self.file_len = 0;
        // println!("{:?}", index);
        
        for (key, value) in index {
            self.set(key, value);
        }
        Ok(())
    }

    /// Set up value by key into KVS
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        if self.file_len - self.storage.len() > 2000 {
            // println!("compaction triggered {}", self.storage.len());
            self.compaction();
        }

        let cmd = KVSCommands::Set {
            key: key.clone(),
            value: value.clone()
        };
        let cmd_str = serde_json::to_string(&cmd)
            .expect("Cant serialize ((");

        // move to end of the file and then write
        let _ = &self.file.seek(SeekFrom::End(0 as i64));
        let pos_result = self.file.stream_position();

        match pos_result {
            Ok(pos) => {
                let _ = &self.file.write_all(cmd_str.as_bytes())
                    .expect("Cant write to file");
                let index = KVSPosition{
                    position: pos,
                    len: cmd_str.len()
                };
                self.storage.insert(key.clone(), index);
                self.file_len += 1;
                Ok(())
            },
            Err(_) => Err("No value, file corrupted".to_owned())
        }
    }
    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>, String> {
        // println!("{:?}", self.storage);
        let record_option = self.storage.get(
            key.as_str()
        );
        match record_option {
            Some(record) => {
                let mut buf_reader = BufReader::new(&self.file);

                let mut buf = String::new();
                buf_reader.seek(SeekFrom::Start(record.position)).expect("Cant seek");
                let mut handle = buf_reader.take(record.len as u64);
                handle.read_to_string(&mut buf).expect("cant read");
                let cmd: KVSCommands = serde_json::from_str(buf.as_str())
                    .expect("cant parse");

                match cmd {
                    KVSCommands::Set {key: _, value} => Ok(Option::from(value)),
                    _ => Err("No value, file corrupted".to_owned())
                }
            }
            None => Ok(None)
        }
    }
    /// Removes value by key
    pub fn remove(&mut self, key: String) -> Result<(), KVSError> {
        let cmd = KVSCommands::Rm { key: key.clone() };
        let cmd_str = serde_json::to_string(&cmd)
            .expect("Cant serialize ((");

        // move to end of the file and then write
        let _ = &self.file.seek(SeekFrom::End(0 as i64));
        let _ =&self.file.write_all(cmd_str.as_bytes())
            .expect("Cant write to file");
        self.file_len += 1;
        match self.storage.remove(key.as_str()) {
            Some(_) => Ok(()),
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
