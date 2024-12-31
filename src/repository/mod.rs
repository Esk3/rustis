use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[cfg(test)]
mod tests;

pub type Repository = LockingMemoryRepository;

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

    pub fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        Ok(self.kv_store.lock().unwrap().get(key).cloned())
    }

    pub fn set(
        &self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<String>> {
        Ok(self.kv_store.lock().unwrap().insert(key, value))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kv_store.lock().unwrap().is_empty()
    }
}
