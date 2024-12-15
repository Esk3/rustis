use std::collections::HashMap;

pub trait Repository {
    fn get(&mut self, key: &str) -> anyhow::Result<Option<&String>>;
    fn set(
        &mut self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<String>>;
}

pub struct MemoryRepository {
    kv_store: HashMap<String, String>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            kv_store: HashMap::new(),
        }
    }
}

impl Repository for MemoryRepository {
    fn get(&mut self, key: &str) -> anyhow::Result<Option<&String>> {
        Ok(self.kv_store.get(key))
    }

    fn set(
        &mut self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    ) -> anyhow::Result<Option<String>> {
        Ok(self.kv_store.insert(key, value))
    }
}
