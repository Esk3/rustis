use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub trait Repository: Clone {
    fn get(&self, key: &str) -> anyhow::Result<Option<String>>;
    fn set(
        &self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<String>>;
}

#[derive(Debug, Clone)]
pub struct LockingMemoryRepository {
    kv_store: Arc<Mutex<HashMap<String, String>>>,
}

impl LockingMemoryRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            kv_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Repository for LockingMemoryRepository {
    fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        Ok(self.kv_store.lock().unwrap().get(key).cloned())
    }

    fn set(
        &self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<String>> {
        Ok(self.kv_store.lock().unwrap().insert(key, value))
    }
}
