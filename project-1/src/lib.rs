use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    storage: HashMap<String, String>
}

/// In memory key value storage String:String
/// Usage
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key1".to_owned(), "value1".to_owned());
/// store.get("key2".to_owned());
/// store.remove("key1".to_owned());
/// #
/// #     Ok(())
/// # }
/// ```
impl KvStore {
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