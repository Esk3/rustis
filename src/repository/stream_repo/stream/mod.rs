#[cfg(test)]
mod tests;

pub mod entry;
pub mod entry_id;
pub mod field;

pub use entry::Entry;
use entry_id::TimestampEntryId;
pub use entry_id::{EntryId, PartialEntryId};
pub use field::Field;

use crate::radix::Radix;

#[derive(Debug)]
pub struct Stream {
    indexes: Radix<usize>,
    entries: Vec<Entry>,
}

impl Stream {
    #[must_use]
    pub fn new() -> Self {
        Self {
            indexes: Radix::new(),
            entries: Vec::new(),
        }
    }

    fn next_id(&self, timestamp: &std::time::SystemTime) -> EntryId {
        self.entries.last().map_or_else(
            || TimestampEntryId::new(timestamp).into_full(),
            |e| e.id.next(timestamp),
        )
    }

    fn min_next_id(&self) -> EntryId {
        self.entries
            .last()
            .map_or(EntryId::min(), |e| &e.id + 1_u64)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indexes.is_empty()
    }

    pub fn add_with_auto_key(
        &mut self,
        fields: Vec<Field>,
        timestamp: &std::time::SystemTime,
    ) -> EntryId {
        let key = self.next_id(timestamp);

        self.indexes
            .add(key.as_radix_key(), self.entries.len())
            .unwrap();
        self.entries.push(Entry::new(key.clone(), fields));
        key
    }

    pub fn try_add_with_key(
        &mut self,
        key: impl PartialEntryId,
        fields: Vec<Field>,
    ) -> anyhow::Result<EntryId> {
        // TODO tests
        let key = key.into_entry_id_or_default(&self.min_next_id());
        if key < self.min_next_id() {
            panic!();
        }
        self.indexes
            .add(key.as_radix_key(), self.entries.len())
            .unwrap();
        self.entries.push(Entry::new(key.clone(), fields));
        Ok(key)
    }

    #[must_use]
    pub fn read(&self, key: &EntryId, count: usize) -> Vec<Entry> {
        let start = match self.entries.binary_search_by_key(key, |e| e.id.clone()) {
            Ok(i) => i + 1,
            Err(i) => i,
        };
        self.entries
            .iter()
            .skip(start)
            .take(count)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn read_last(&self) -> Option<Entry> {
        self.entries.last().cloned()
    }

    #[must_use]
    pub fn range(&self, start: &EntryId, end: &EntryId) -> Vec<Entry> {
        let start = match self.entries.binary_search_by_key(start, |e| e.id.clone()) {
            Ok(i) => i,
            Err(i) => i,
        };

        let end = match self.entries.binary_search_by_key(end, |e| e.id.clone()) {
            Ok(i) => i + 1,
            Err(i) => i,
        };

        self.entries.iter().skip(start).take(end).cloned().collect()
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}
