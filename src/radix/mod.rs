use crate::repository::stream_repo::stream::EntryId;

#[cfg(test)]
mod tests;

/// Append only radix
#[derive(Debug)]
pub enum Radix<V> {
    Node { edge: Vec<u8>, children: Vec<Self> },
    Leaf { edge: Vec<u8>, value: V },
}

impl<V> Radix<V> {
    #[must_use]
    pub fn new() -> Self {
        Self::Node {
            edge: Vec::new(),
            children: Vec::new(),
        }
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Radix::Node { edge: _, children } => children.is_empty(),
            Radix::Leaf { .. } => false,
        }
    }

    pub fn add(&mut self, key: &[u8], value: V) -> Result<(), V> {
        match self {
            Radix::Node { edge: _, children } => {
                if children.iter().any(|child| child.edge() == key) {
                    return Err(value);
                }
                if let Some(next) = children
                    .iter_mut()
                    .find(|child| key.common_prefix(child.edge()).is_some())
                {
                    match next {
                        Radix::Node { edge, children } => {
                            let key = key.strip_common_prefix(edge);
                            next.add(key, value)
                        }
                        Radix::Leaf { edge: _, value: _ } => {
                            let Radix::Leaf {
                                edge,
                                value: leaf_value,
                            } = std::mem::take(next)
                            else {
                                unreachable!()
                            };
                            let common = key.common_prefix(&edge).unwrap();
                            let n1 = Self::Leaf {
                                edge: edge.strip_common_prefix(common).to_vec(),
                                value: leaf_value,
                            };
                            let n2 = Self::Leaf {
                                edge: key.strip_common_prefix(common).to_vec(),
                                value,
                            };
                            *next = Self::Node {
                                edge: common.to_vec(),
                                children: vec![n1, n2],
                            };
                            Ok(())
                        }
                    }
                } else {
                    children.push(Self::Leaf {
                        edge: key.to_vec(),
                        value,
                    });
                    Ok(())
                }
            }
            Radix::Leaf { edge, value: _ } => {
                // same prefix
                // if this is empty make other child of this
                // if other is empty make this child of other
                // find common prefix and make both children of new node
                assert!(!(edge.is_empty() && key.is_empty()), "duplicate i think");
                assert!(!edge.is_empty());
                assert!(!key.is_empty());
                *self = Self::new();
                todo!("");
            }
        }
    }

    fn edge(&self) -> &[u8] {
        match self {
            Radix::Node { edge, children: _ } | Radix::Leaf { edge, value: _ } => edge,
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&V> {
        match self {
            Radix::Node { edge, children } => {
                let child = children
                    .iter()
                    .find(|child| child.edge().common_prefix(key).is_some())?;
                match child {
                    Radix::Node { edge, children } => child.get(key.strip_common_prefix(edge)),
                    Radix::Leaf { edge, value } if edge == key => Some(value),
                    Radix::Leaf { .. } => None,
                }
            }
            Radix::Leaf { edge, value } => todo!(),
        }
    }

    fn get_next_node(&self, key: &[u8]) -> Option<&Self> {
        match self {
            Radix::Node { edge, children } => {
                let child = children
                    .iter()
                    .find(|child| child.edge().common_prefix(key).is_some())?;
                match child {
                    Radix::Node { edge, children } => Some(child),
                    Radix::Leaf { edge, value } if edge == key => todo!(),
                    Radix::Leaf { .. } => None,
                }
            }
            Radix::Leaf { edge, value } => todo!(),
        }
    }
}

impl<T> Default for Radix<T> {
    fn default() -> Self {
        Self::new()
    }
}

trait RadixCmp {
    fn find_common_prefix(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl RadixCmp for &str {
    fn find_common_prefix(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        let count = self
            .chars()
            .zip(other.chars())
            .take_while(|(this, other)| this == other)
            .count();
        if count > 0 {
            Some(&self[0..count])
        } else {
            None
        }
    }
}

struct RadixStreamIdKey {
    key: [u8; std::mem::size_of::<u64>() * 8 * 2],
}

impl RadixCmp for RadixStreamIdKey {
    fn find_common_prefix(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl From<EntryId> for RadixStreamIdKey {
    fn from(value: EntryId) -> Self {
        todo!()
    }
}

pub trait IntoRadixKey {
    fn into_key(self) -> Vec<u8>;
}

impl IntoRadixKey for String {
    fn into_key(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoRadixKey for u64 {
    fn into_key(self) -> Vec<u8> {
        self.to_string().bytes().map(|b| b - b'0').collect()
    }
}

impl IntoRadixKey for Vec<u8> {
    fn into_key(self) -> Vec<u8> {
        self
    }
}

pub trait CommondPrefix {
    fn common_prefix(&self, other: &[u8]) -> Option<&[u8]>;
    fn strip_common_prefix(&self, other: &[u8]) -> &[u8];
}

impl CommondPrefix for [u8] {
    fn common_prefix(&self, other: &[u8]) -> Option<&[u8]> {
        let count = self.iter().zip(other).take_while(|(a, b)| a == b).count();
        if count == 0 {
            return None;
        }
        Some(&self[0..count])
    }
    fn strip_common_prefix(&self, other: &[u8]) -> &[u8] {
        let split = self
            .common_prefix(other)
            .map_or(0, <[u8]>::len);
        &self[split..]
    }
}
