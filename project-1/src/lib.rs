use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    storage: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        Self {storage: HashMap::new()}
    }

    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.storage.get(key.as_str()).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.storage.remove(key.as_str());
    }
}