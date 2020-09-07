use std::collections::HashMap;

pub trait KvStore {
    fn new() -> Self;
    fn set(&mut self, key: String, value: String);
    fn get(&mut self, key: String) -> Option<String>;
    fn remove(&mut self, key: String);
}

pub struct KvStoreMap {
    map_store: HashMap<String, String>,
}

impl KvStore for KvStoreMap {
    fn new() -> Self {
        Self {
            map_store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map_store.insert(key, value);
    }

    fn get(&mut self, key: String) -> Option<String> {
        return self.map_store.get(&key).cloned();
    }

    fn remove(&mut self, key: String) {
        self.map_store.remove(&key);
    }
}
