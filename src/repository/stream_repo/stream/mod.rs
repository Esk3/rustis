#[cfg(test)]
mod tests;

pub mod entry_id;

use entry_id::TimestampEntryId;
pub use entry_id::{EntryId, PartialEntryId};

use crate::{
    radix::Radix,
    resp::{self, value::IntoRespArray},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    name: String,
    value: String,
}

impl Field {
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub(super) id: EntryId,
    pub(super) fields: Vec<Field>,
}

impl Entry {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(id: EntryId, fields: Vec<Field>) -> Self {
        Self { id, fields }
    }

    #[must_use]
    pub fn id(&self) -> &EntryId {
        &self.id
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

impl From<Entry> for resp::Value {
    fn from(value: Entry) -> Self {
        [
            resp::Value::simple_string(value.id),
            value
                .fields
                .into_iter()
                .map(|field| [field.name, field.value])
                .flatten()
                .map(resp::Value::simple_string)
                .collect(),
        ]
        .into_array()
    }
}

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

    pub fn add_with_partial_key(
        &mut self,
        key: impl PartialEntryId,
        value: impl ToString,
    ) -> EntryId {
        todo!()
    }

    pub fn add_default_key(&mut self, key: impl PartialEntryId, value: impl ToString) -> EntryId {
        todo!()
        //let key = key.into_entry_id_or_default(&self.next);
        //if key == self.next {
        //    self.next.id += 1;
        //}
        //if key > self.next {
        //    self.next = key.clone();
        //    self.next.id += 1;
        //}
        //self.indexes
        //    .add(key.as_radix_key(), self.entries.len())
        //    .unwrap();
        //self.entries.push(Entry::new(key.clone(), value));
        //key
    }

    #[must_use]
    pub fn read(&self, key: &EntryId, count: usize) -> Vec<Entry> {
        let start = match self.entries.binary_search_by_key(key, |e| e.id.clone()) {
            Ok(i) => i,
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
