pub mod kv_repo;
pub mod stream_repo;

#[derive(Debug, Clone)]
pub struct Repository {
    kv_repo: kv_repo::KvRepository,
    stream_repo: stream_repo::StreamRepository,
}

impl Repository {
    #[must_use]
    pub fn new(kv_repo: kv_repo::KvRepository, stream_repo: stream_repo::StreamRepository) -> Self {
        Self {
            kv_repo,
            stream_repo,
        }
    }
    #[must_use]
    pub fn kv_repo(&self) -> &kv_repo::KvRepository {
        &self.kv_repo
    }

    #[must_use]
    pub fn stream_repo(&self) -> &stream_repo::StreamRepository {
        &self.stream_repo
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            kv_repo: kv_repo::KvRepository::new(),
            stream_repo: stream_repo::StreamRepository::new(),
        }
    }
}
