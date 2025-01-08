pub mod kv_repo;
pub mod stream_repo;

#[derive(Debug, Clone)]
pub struct Repository {
    kv_repo: kv_repo::LockingMemoryRepository,
    stream_repo: stream_repo::LockingStreamRepository,
}

impl Repository {
    #[must_use]
    pub fn new(
        kv_repo: kv_repo::LockingMemoryRepository,
        stream_repo: stream_repo::LockingStreamRepository,
    ) -> Self {
        Self {
            kv_repo,
            stream_repo,
        }
    }
    #[must_use]
    pub fn kv_repo(&self) -> &kv_repo::LockingMemoryRepository {
        &self.kv_repo
    }

    #[must_use]
    pub fn stream_repo(&self) -> &stream_repo::LockingStreamRepository {
        &self.stream_repo
    }

    pub fn get(
        &self,
        key: &str,
        timestamp: std::time::SystemTime,
    ) -> anyhow::Result<Option<String>> {
        self.kv_repo.get(key, timestamp)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kv_repo.is_empty()
    }

    pub fn set(
        &self,
        key: String,
        value: String,
        expiry: Option<std::time::SystemTime>,
    ) -> anyhow::Result<Option<String>> {
        self.kv_repo.set(key, value, expiry)
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            kv_repo: kv_repo::LockingMemoryRepository::new(),
            stream_repo: stream_repo::LockingStreamRepository::new(),
        }
    }
}
