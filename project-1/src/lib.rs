#![deny(missing_docs)]
//! Module with key-value storage
use std::collections::HashMap;

/// In memory key value storage String:String
#[derive(Debug)]
pub struct KvStore {
    storage: HashMap<String, String>
}

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

impl KvStore {
    /// Create new instance
    pub fn new() -> Self {
        Self {storage: HashMap::new()}
    }
    /// Set up value by key into KVS
    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }
    /// Get value by key
    pub fn get(&self, key: String) -> Option<String> {
        self.storage.get(key.as_str()).cloned()
    }
    /// Removes value by key
    pub fn remove(&mut self, key: String) {
        self.storage.remove(key.as_str());
    }
}