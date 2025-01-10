pub mod deserialize;
pub mod identifier;
pub mod serialize;

use anyhow::anyhow;
pub use deserialize::deserialize_value;
pub use serialize::serialize_value;

#[derive(Debug, Clone)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    BulkByteString(Vec<u8>),
    NullString,

    Array(Vec<Self>),
    NullArray,
}

impl Value {
    pub fn simple_string(s: impl ToString) -> Self {
        Self::SimpleString(s.to_string())
    }
    pub fn bulk_string(s: impl ToString) -> Self {
        Self::BulkString(s.to_string())
    }
    pub fn bulk_strings(s: impl ToString) -> Vec<Self> {
        Self::bulk_string_pat(s, ";")
    }
    pub fn bulk_string_pat(s: impl ToString, pat: &str) -> Vec<Self> {
        s.to_string()
            .split(pat)
            .map(str::trim)
            .map(Self::bulk_string)
            .collect()
    }
    #[must_use]
    pub fn ok() -> Self {
        Self::simple_string("OK")
    }
    pub fn into_string(self) -> Result<String, Self> {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => Ok(s),
            Value::NullString | Value::NullArray | Value::BulkByteString(_) | Value::Array(_) => {
                Err(self)
            }
        }
    }
    pub fn expect_string(self) -> anyhow::Result<String> {
        self.into_string()
            .map_err(|err| anyhow!("expected string got {err:?}"))
    }
    pub fn into_array(self) -> Result<Vec<Self>, Self> {
        if let Self::Array(arr) = self {
            Ok(arr)
        } else {
            Err(self)
        }
    }

    pub fn into_byte_string(self) -> Result<Vec<u8>, Self> {
        match self {
            Value::SimpleString(_) => Err(self),
            Value::BulkString(_) => todo!(),
            Value::BulkByteString(bytes) => Ok(bytes),
            Value::NullString => todo!(),
            Value::Array(_) => todo!(),
            Value::NullArray => todo!(),
        }
    }
    #[must_use]
    pub fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => s.eq_ignore_ascii_case(other),
            Value::NullString | Value::NullArray | Value::BulkByteString(_) | Value::Array(_) => {
                false
            }
        }
    }
}

pub trait IntoRespArray {
    fn into_array(self) -> Value;
}

impl IntoRespArray for Vec<Value> {
    fn into_array(self) -> Value {
        Value::Array(self)
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<Self>>().into_array()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SimpleString(l0), Self::SimpleString(r0))
            | (Self::BulkString(l0), Self::BulkString(r0)) => l0 == r0,
            (Self::SimpleString(l0) | Self::BulkString(l0), Self::BulkByteString(r0)) => {
                l0.as_bytes() == r0
            }
            (Self::BulkByteString(l0), Self::SimpleString(r0) | Self::BulkString(r0)) => {
                l0 == r0.as_bytes()
            }
            (Self::BulkByteString(l0), Self::BulkByteString(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Value {}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => other == s,
            Value::NullString | Value::NullArray | Value::BulkByteString(_) | Value::Array(_) => {
                false
            }
        }
    }
}

impl From<&[Value]> for Value {
    fn from(value: &[Value]) -> Self {
        Self::Array(value.to_vec())
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        Self::BulkByteString(value.to_vec())
    }
}
