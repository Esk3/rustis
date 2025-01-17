use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[cfg(test)]
mod tests;

pub type KvRepository = LockingMemoryRepository;

#[derive(Debug, Clone)]
pub struct LockingMemoryRepository {
    kv_store: Arc<Mutex<HashMap<String, String>>>,
    kv_store_expiry: Arc<Mutex<HashMap<String, std::time::SystemTime>>>,
}

impl LockingMemoryRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            kv_store: Arc::new(Mutex::new(HashMap::new())),
            kv_store_expiry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(
        &self,
        key: &str,
        timestamp: std::time::SystemTime,
    ) -> anyhow::Result<Option<String>> {
        let mut expiry_lock = self.kv_store_expiry.lock().unwrap();
        let mut store_lock = self.kv_store.lock().unwrap();
        let key_has_expiry_time_and_is_expired = expiry_lock
            .get(key)
            .is_some_and(|expiry| *expiry < timestamp);
        if key_has_expiry_time_and_is_expired {
            expiry_lock.remove(key);
            store_lock.remove(key);
            Ok(None)
        } else {
            Ok(store_lock.get(key).cloned())
        }
    }

    pub fn set(
        &self,
        key: String,
        value: String,
        expiry: Option<std::time::SystemTime>,
    ) -> anyhow::Result<Option<String>> {
        let mut lock = self.kv_store_expiry.lock().unwrap();
        if let Some(expiry) = expiry {
            lock.insert(key.clone(), expiry);
        } else {
            lock.remove(&key);
        }
        Ok(self.kv_store.lock().unwrap().insert(key, value))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        let is_empty = self.kv_store.lock().unwrap().is_empty();
        if is_empty {
            assert!(
                self.kv_store_expiry.lock().unwrap().is_empty(),
                "expiry should be empty if key value store is empty"
            );
        }
        is_empty
    }
}

impl Default for LockingMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}
