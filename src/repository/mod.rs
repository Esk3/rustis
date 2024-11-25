use std::{collections::HashMap, hash::Hash};

pub struct Repository<K = String, V = String> {
    kv_store: KvStore<K, V>,
}

impl Repository {
    pub fn get(&mut self, key: String, timestamp: &std::time::SystemTime) -> Option<&String> {
        self.kv_store.get(key, timestamp)
    }
    pub fn set(
        &mut self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
        timestamp: &std::time::SystemTime,
    ) -> Option<String> {
        self.kv_store.set(key, value, expiry, timestamp)
    }
}

pub struct KvStore<K, V> {
    store: HashMap<K, V>,
    expiries: HashMap<K, std::time::SystemTime>,
}

impl<K, V> KvStore<K, V>
where
    K: Hash + PartialEq + Eq,
{
    pub fn get(&mut self, key: K, timestamp: &std::time::SystemTime) -> Option<&V> {
        match self.expiries.entry(key) {
            std::collections::hash_map::Entry::Occupied(timeout) if timeout.get() < timestamp => {
                let key = timeout.key();
                self.store.get(key)
            }
            std::collections::hash_map::Entry::Vacant(e) => self.store.get(e.key()),
            std::collections::hash_map::Entry::Occupied(e) => {
                let (key, _) = e.remove_entry();
                self.store.remove(&key);
                None
            }
        }
    }
    pub fn set(
        &mut self,
        key: K,
        value: V,
        expiry: Option<std::time::Duration>,
        timestamp: &std::time::SystemTime,
    ) -> Option<V>
    where
        K: Clone,
    {
        if let Some(expiry) = expiry {
            self.expiries.insert(key.clone(), *timestamp + expiry);
        } else {
            self.expiries.remove(&key);
        };
        self.store.insert(key, value)
    }
}
