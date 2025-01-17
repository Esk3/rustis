use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{bail, Context};
use stream::{Entry, EntryId, PartialEntryId, Stream};

pub use block_result::BlockResult;

pub mod block_result;
pub mod stream;

#[cfg(test)]
pub mod tests;

pub type StreamRepository = LockingStreamRepository;

#[derive(Debug, Clone)]
pub struct LockingStreamRepository {
    streams: Arc<Mutex<HashMap<String, Stream>>>,
    listners: Arc<Mutex<HashMap<String, Vec<std::sync::mpsc::Sender<Event>>>>>,
}

impl LockingStreamRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            streams: Arc::new(Mutex::new(HashMap::new())),
            listners: Arc::default(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_auto_increment(
        &self,
        stream_key: impl ToString,
        fields: Vec<stream::Field>,
        timestamp: &std::time::SystemTime,
    ) -> EntryId {
        self.notify_add(stream_key, |stream| {
            stream.add_with_auto_key(fields, timestamp)
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add(
        &self,
        stream_key: impl ToString,
        entry_id: impl PartialEntryId,
        fields: Vec<stream::Field>,
    ) -> anyhow::Result<EntryId> {
        self.notify_add(stream_key, |stream| {
            stream.try_add_with_key(entry_id, fields)
        })
    }

    fn notify_add<F, T>(&self, stream_key: impl ToString, f: F) -> T
    where
        F: FnOnce(&mut Stream) -> T,
    {
        let mut lock = self.streams.lock().unwrap();
        let stream = lock.entry(stream_key.to_string()).or_default();
        self.wakeup_listers("0");
        f(stream)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn read(
        &self,
        stream_key: impl ToString,
        entry_id: &EntryId,
        count: usize,
    ) -> anyhow::Result<Vec<Entry>> {
        let lock = self.streams.lock().unwrap();
        let Some(stream) = lock.get(&stream_key.to_string()) else {
            bail!("stream not found")
        };
        Ok(stream.read(entry_id, count))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn read_last(&self, stream_key: impl ToString) -> anyhow::Result<Entry> {
        let lock = self.streams.lock().unwrap();
        let Some(stream) = lock.get(&stream_key.to_string()) else {
            todo!();
        };
        stream.read_last().context("stream empty")
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn range(
        &self,
        stream_key: impl ToString,
        start: &EntryId,
        end: &EntryId,
    ) -> anyhow::Result<Vec<Entry>> {
        let lock = self.streams.lock().unwrap();
        let Some(stream) = lock.get(&stream_key.to_string()) else {
            bail!("stream not found")
        };
        Ok(stream.range(start, end))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn read_blocking(
        &self,
        stream_key: impl ToString,
        entry_id: &EntryId,
        count: usize,
        block_duration: Option<std::time::Duration>,
    ) -> BlockResult<Vec<Entry>> {
        self.blocking_query(block_duration, |_repo| -> BlockResult<Vec<Entry>> {
            let res = self.read(stream_key.to_string(), entry_id, count).unwrap();
            if res.is_empty() {
                BlockResult::NotFound
            } else {
                BlockResult::Found(res)
            }
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn range_blocking(
        &self,
        stream_key: impl ToString,
        start: &EntryId,
        end: &EntryId,
        block_duration: Option<std::time::Duration>,
    ) -> BlockResult<Vec<Entry>> {
        self.blocking_query(block_duration, |_repo| {
            let res = self.range(stream_key.to_string(), start, end).unwrap();
            if res.is_empty() {
                BlockResult::NotFound
            } else {
                BlockResult::Found(res)
            }
        })
    }

    fn blocking_query<F, T>(
        &self,
        block_duration: Option<std::time::Duration>,
        f: F,
    ) -> BlockResult<T>
    where
        F: Fn(&Self) -> BlockResult<T>,
    {
        let result = f(self);
        if result.is_not_found() {
            self.block(block_duration, f)
        } else {
            result
        }
    }

    fn get_listner(&self, stream: impl ToString) -> std::sync::mpsc::Receiver<Event> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.listners
            .lock()
            .unwrap()
            .entry(stream.to_string())
            .or_default()
            .push(tx);
        rx
    }

    fn get_listner_with_timeout(
        &self,
        stream: impl ToString,
        block_duration: std::time::Duration,
    ) -> std::sync::mpsc::Receiver<Event> {
        let (tx, rx) = std::sync::mpsc::channel();

        self.listners
            .lock()
            .unwrap()
            .entry(stream.to_string())
            .or_default()
            .push(tx.clone());

        std::thread::spawn(move || {
            std::thread::sleep(block_duration);
            _ = tx.send(Event::Timeout);
        });
        rx
    }

    fn block<F, T>(&self, block_duration: Option<std::time::Duration>, f: F) -> BlockResult<T>
    where
        F: Fn(&Self) -> BlockResult<T>,
    {
        let rx = if let Some(block_duration) = block_duration {
            self.get_listner_with_timeout(0, block_duration)
        } else {
            self.get_listner(0)
        };

        for event in rx {
            match event {
                Event::Added => {
                    let result = f(self);
                    if !result.is_not_found() {
                        return result;
                    }
                }
                Event::Timeout => return BlockResult::NotFound,
            }
        }

        BlockResult::NotFound
    }

    fn wakeup_listers(&self, stream: &str) {
        let mut lock = self.listners.lock().unwrap();
        let Some(listners) = lock.get_mut(stream) else {
            return;
        };
        listners.retain(|sender| sender.send(Event::Added).is_ok());
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.streams.lock().unwrap().is_empty()
    }
}

impl Default for LockingStreamRepository {
    fn default() -> Self {
        Self::new()
    }
}

enum Event {
    Added,
    Timeout,
}
