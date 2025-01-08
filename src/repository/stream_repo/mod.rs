use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{bail, Context};
use stream::Stream;

pub mod stream;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct LockingStreamRepository {
    streams: Arc<Mutex<HashMap<String, Stream>>>,
    listners: Arc<Mutex<HashMap<String, std::sync::mpsc::Sender<Event>>>>,
}

impl LockingStreamRepository {
    #[must_use]
    pub fn new() -> Self {
        Self {
            streams: Arc::new(Mutex::new(HashMap::new())),
            listners: Arc::default(),
        }
    }

    pub fn xadd(
        &self,
        stream_key: impl ToString,
        entry_id: Option<StreamId>,
        value: impl ToString,
    ) -> anyhow::Result<String> {
        let mut lock = self.streams.lock().unwrap();
        let key = lock
            .entry(stream_key.to_string())
            .or_insert(Stream::new())
            .add_default_key("ConstDefaultKey", value);
        self.listners
            .lock()
            .unwrap()
            .get(&0.to_string())
            .map(|tx| tx.send(Event::Added));
        Ok(key)
    }

    pub fn xread(
        &self,
        stream_key: impl ToString,
        entry_id: impl ToString,
        count: usize,
    ) -> anyhow::Result<Vec<String>> {
        let lock = self.streams.lock().unwrap();
        let Some(stream) = lock.get(&stream_key.to_string()) else {
            bail!("stream not found")
        };
        Ok(stream.read(entry_id.to_string(), count))
    }

    pub fn xread_last(&self, stream_key: impl ToString) -> anyhow::Result<String> {
        let lock = self.streams.lock().unwrap();
        let Some(stream) = lock.get(&stream_key.to_string()) else {
            todo!();
        };
        stream.read_last().context("stream empty")
    }

    pub fn xread_blocking(&self) {
        todo!()
    }

    pub fn xrange(
        &self,
        stream_key: impl ToString,
        start: impl ToString,
        end: impl ToString,
    ) -> anyhow::Result<Vec<String>> {
        let lock = self.streams.lock().unwrap();
        let Some(stream) = lock.get(&stream_key.to_string()) else {
            bail!("stream not found")
        };
        Ok(stream.range(start, end))
    }

    pub fn blocking_query<F, T>(
        &self,
        block_duration: std::time::Duration,
        f: F,
    ) -> Option<anyhow::Result<T>>
    where
        F: Fn(&Self) -> anyhow::Result<Option<T>>,
    {
        match f(self) {
            Ok(Some(res)) => Some(Ok(res)),
            Ok(None) => {
                let (tx, rx) = std::sync::mpsc::channel();
                self.listners
                    .lock()
                    .unwrap()
                    .insert(0.to_string(), tx.clone());
                std::thread::spawn(move || {
                    std::thread::sleep(block_duration);
                    _ = tx.send(Event::Timeout);
                });
                for event in rx {
                    match event {
                        Event::Added => match f(self) {
                            Ok(Some(res)) => return Some(Ok(res)),
                            Ok(None) => todo!(),
                            Err(_) => todo!(),
                        },
                        Event::Timeout => return None,
                    }
                }
                None
            }
            Err(err) => Some(Err(err)),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.streams.lock().unwrap().is_empty()
    }
}

pub struct StreamId;

enum Event {
    Added,
    Timeout,
}
