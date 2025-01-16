use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{bail, Context};
use stream::{Entry, EntryId, Stream};

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
        let mut lock = self.streams.lock().unwrap();
        let stream = lock.entry(stream_key.to_string()).or_insert(Stream::new());
        self.wakeup_listers("0");
        stream.add_with_auto_key(fields, timestamp)
    }

    //#[allow(clippy::needless_pass_by_value)]
    //pub fn xadd(
    //    &self,
    //    stream_key: impl ToString,
    //    entry_id: impl PartialEntryId,
    //    value: impl ToString,
    //) -> anyhow::Result<EntryId> {
    //    let mut lock = self.streams.lock().unwrap();
    //    let key = lock
    //        .entry(stream_key.to_string())
    //        .or_insert(Stream::new())
    //        .add_default_key(EntryId::min(), value);
    //    self.wakeup_listers("0");
    //    Ok(key)
    //}

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
        block_duration: std::time::Duration,
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
        block_duration: std::time::Duration,
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

    fn blocking_query<F, T>(&self, block_duration: std::time::Duration, f: F) -> BlockResult<T>
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

    fn block<F, T>(&self, block_duration: std::time::Duration, f: F) -> BlockResult<T>
    where
        F: Fn(&Self) -> BlockResult<T>,
    {
        let rx = self.get_listner_with_timeout(0, block_duration);

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

enum Event {
    Added,
    Timeout,
}

#[derive(Debug)]
pub enum BlockResult<T> {
    Found(T),
    NotFound,
    Err(anyhow::Error),
}

impl<T: Eq> Eq for BlockResult<T> {}

impl<T: PartialEq> PartialEq for BlockResult<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Found(l0), Self::Found(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<T> BlockResult<T> {
    fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }
}
