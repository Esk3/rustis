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
    pub fn new(timestamp: u64, id: u64) -> Self {
        let key = timestamp
            .to_string()
            .as_bytes()
            .into_iter()
            .map(|b| b - b'0')
            .chain(id.to_string().as_bytes().into_iter().map(|b| b - b'0'))
            .collect();
        Self { timestamp, id, key }
    }
    pub fn min() -> Self {
        Self::new(0, 1)
    }
    pub fn max() -> Self {
        Self::new(u64::MAX, u64::MAX)
    }
    pub fn to_string(&self) -> String {
        todo!()
    }
    pub fn as_radix_key(&self) -> &[u8] {
        &self.key
    }
}

pub trait DefaultEntryId {
    fn into_or_default(self, default: &EntryId) -> EntryId;
}

pub struct EmptyEntryId;
pub struct PartialEntryId {
    timestamp: u64,
}

impl DefaultEntryId for EntryId {
    fn into_or_default(self, _default: &EntryId) -> EntryId {
        self
    }
}
impl DefaultEntryId for EmptyEntryId {
    fn into_or_default(self, default: &EntryId) -> EntryId {
        default.clone()
    }
}
impl DefaultEntryId for PartialEntryId {
    fn into_or_default(self, default: &EntryId) -> EntryId {
        EntryId::new(self.timestamp, default.id)
    }
}
