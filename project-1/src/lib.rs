pub struct KvStore {

}

impl KvStore {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set(&self, key: String, value: String) {
    }

    pub fn get(&self, key: String) -> Option<String> {
        Some("Some".to_owned())
    }

    pub fn remove(&self, key: String) {
    }
}