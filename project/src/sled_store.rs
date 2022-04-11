// #![deny(missing_docs)]
//! Sled engine implementation
use sled::Db;
use std::path::PathBuf;

use crate::engine::KvsEngine;
use crate::error::{KVSError, Result};

const DATABASE_FILENAME: &str = "sled.db";

/// Usage
/// ```rust
/// # use std::error::Error;
/// # use assert_cmd::prelude::*;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use std::path::Path;
/// use crate::kvs::KvsEngine;
/// use kvs::SledStore;
///
/// let mut path = Path::new(".");
///
/// let mut store = SledStore::open(path).unwrap();
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
pub struct SledStore {
    tree: Db,
}

impl KvsEngine for SledStore {
    /// Set up value by key into KVS
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.tree.insert(key, value.as_bytes())?;
        self.tree.flush()?;
        Ok(())
    }
    /// Get value by key
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let val_ivec = self.tree.get(&key)?;

        match val_ivec {
            Some(ivec) => {
                let value = String::from_utf8(ivec.to_vec())?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    /// Removes value by key
    fn remove(&mut self, key: String) -> Result<()> {
        if let Ok(old_value_option) = self.tree.remove(&key) {
            match old_value_option {
                Some(_v) => {
                    self.tree.flush()?;
                    Ok(())
                }
                None => Err(KVSError::GeneralKVSError),
            }
        } else {
            Err(KVSError::GeneralKVSError)
        }
    }
}

impl SledStore {
    /// Create new instance of Sled
    pub fn new(path: PathBuf) -> Result<Self> {
        let tree = sled::open(path)?;
        let obj = Self { tree };
        Ok(obj)
    }

    /// Open the Sled at a given path. Return the Sled tree.
    pub fn open(path: impl Into<PathBuf>) -> Result<SledStore> {
        let mut path_buf = path.into();
        path_buf.push(DATABASE_FILENAME);
        SledStore::new(path_buf)
    }
}
