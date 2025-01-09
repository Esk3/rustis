#[cfg(test)]
mod tests;

pub mod entry_id;

pub use entry_id::*;

use crate::radix::Radix;

#[derive(Debug, Clone)]
struct Entry {
    id: EntryId,
    value: String,
}

impl Entry {
    fn new(id: EntryId, value: impl ToString) -> Self {
        Self {
            id,
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Stream {
    indexes: Radix<usize>,
    entries: Vec<Entry>,
    next: EntryId,
}

impl Stream {
    #[must_use]
    pub fn new() -> Self {
        Self {
            indexes: Radix::new(),
            entries: Vec::new(),
            next: EntryId::min(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indexes.is_empty()
    }

    pub fn add_default_key(&mut self, key: impl DefaultEntryId, value: impl ToString) -> EntryId {
        let key = key.into_or_default(&self.next);
        if key == self.next {
            self.next.id += 1;
        }
        if key > self.next {
            self.next = key.clone();
            self.next.id += 1;
        }
        self.indexes.add(key.as_radix_key(), self.entries.len());
        self.entries.push(Entry::new(key.clone(), value));
        key
    }

    pub fn read(&self, key: &EntryId, count: usize) -> Vec<String> {
        let start = match self.entries.binary_search_by_key(key, |e| e.id.clone()) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.entries[start..]
            .into_iter()
            .map(|e| e.value.clone())
            .collect()
    }

    #[must_use]
    pub fn read_last(&self) -> Option<String> {
        self.entries.last().map(|e| e.value.clone())
    }

    #[must_use]
    pub fn range(&self, start: &EntryId, end: &EntryId) -> Vec<String> {
        let start = match self.entries.binary_search_by_key(start, |e| e.id.clone()) {
            Ok(i) => i,
            Err(i) => i,
        };
        let end = match self.entries.binary_search_by_key(end, |e| e.id.clone()) {
            Ok(i) => i + 1,
            Err(i) => i,
        };
        self.entries[start..end]
            .iter()
            .map(|e| e.value.to_string())
            .collect()
    }
}
