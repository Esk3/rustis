use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryId {
    pub timestamp: u64,
    pub id: u64,
    key: Vec<u8>,
}

impl Ord for EntryId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.timestamp.cmp(&other.timestamp) {
            core::cmp::Ordering::Equal => self.id.cmp(&other.id),
            ord => ord,
        }
    }
}

impl PartialOrd for EntryId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl EntryId {
    #[must_use]
    pub fn new(timestamp: u64, id: u64) -> Self {
        let key = timestamp
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .chain(id.to_string().as_bytes().iter().map(|b| b - b'0'))
            .collect();
        Self { timestamp, id, key }
    }

    #[must_use]
    pub fn min() -> Self {
        Self::new(0, 1)
    }

    #[must_use]
    pub fn max() -> Self {
        Self::new(u64::MAX, u64::MAX)
    }

    /// # Safety
    /// this function creates an invalid entry id.
    /// the entry id gotten from calling `Self::next` on this will be valid
    /// so only to be used for comparisions or to call `Self::next` on to get the first valid id
    #[must_use]
    pub unsafe fn null() -> Self {
        Self::new(0, 0)
    }

    #[must_use]
    pub fn next(&self, timestamp: &std::time::SystemTime) -> Self {
        let timestamp = timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap();
        let timestamp = self.timestamp.max(timestamp);
        let id = if timestamp == self.timestamp {
            self.id + 1
        } else {
            0
        };
        let key = format!("{timestamp}{id}").as_bytes().to_vec();
        Self { timestamp, id, key }
    }

    #[must_use]
    pub fn as_radix_key(&self) -> &[u8] {
        &self.key
    }
}

impl Display for EntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::ops::Add<u64> for EntryId {
    type Output = EntryId;

    fn add(self, rhs: u64) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Add<u64> for &EntryId {
    type Output = EntryId;

    fn add(self, rhs: u64) -> Self::Output {
        // TODO add to timestamp if id overflows
        EntryId::new(self.timestamp, self.id.checked_add(rhs).unwrap())
    }
}

pub trait PartialEntryId {
    fn into_entry_id_or_default(self, default: &EntryId) -> EntryId;
    fn try_into_full_entry_id(self) -> Option<EntryId>;
}

pub struct EmptyEntryId;

impl EmptyEntryId {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

pub struct TimestampEntryId {
    timestamp: u64,
}

impl TimestampEntryId {
    #[must_use]
    pub fn new(timestamp: &std::time::SystemTime) -> Self {
        Self {
            timestamp: timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .try_into()
                .unwrap(),
        }
    }

    #[must_use]
    pub fn from_millis(millis: u64) -> Self {
        Self { timestamp: millis }
    }

    #[must_use]
    pub fn into_full(self) -> EntryId {
        if self.timestamp > 0 {
            EntryId::new(self.timestamp, 0)
        } else {
            EntryId::new(0, 1)
        }
    }
}

impl PartialEntryId for EntryId {
    fn into_entry_id_or_default(self, _default: &EntryId) -> EntryId {
        self
    }

    fn try_into_full_entry_id(self) -> Option<EntryId> {
        Some(self)
    }
}

impl PartialEntryId for EmptyEntryId {
    fn into_entry_id_or_default(self, default: &EntryId) -> EntryId {
        default.clone()
    }

    fn try_into_full_entry_id(self) -> Option<EntryId> {
        None
    }
}

impl PartialEntryId for TimestampEntryId {
    fn into_entry_id_or_default(self, default: &EntryId) -> EntryId {
        EntryId::new(self.timestamp, default.id)
    }

    fn try_into_full_entry_id(self) -> Option<EntryId> {
        None
    }
}
