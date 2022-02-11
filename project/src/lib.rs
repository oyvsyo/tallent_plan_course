#![deny(missing_docs)]
//! Module with key-value storage
use std::collections::HashMap;
use std::path::PathBuf;

/// Usage
/// ```rust
/// # use std::error::Error;
/// # use assert_cmd::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key1".to_owned(), "value1".to_owned());
/// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
/// store.remove("key1".to_owned());
/// #
/// #     Ok(())
/// # }
/// ```

/// In memory key value storage String:String
#[derive(Debug)]
pub struct KvStore {
    storage: HashMap<String, String>,
    file_path: PathBuf
}

impl KvStore {
    /// Create new instance
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            storage: HashMap::new(),
            file_path: path
        }
    }
    /// Set up value by key into KVS
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        self.storage.insert(key, "set".to_string());
        Ok(())
    }
    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>, String> {
        Ok(self.storage.get(key.as_str()).cloned())
    }
    /// Removes value by key
    pub fn remove(&mut self, key: String) -> Result<(), String> {
        self.storage.remove(key.as_str());
        Ok(())
    }
    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore, String> {
        Ok(KvStore::new())
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
