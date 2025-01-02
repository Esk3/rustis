#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    BulkByteString(Vec<u8>),
    NullString,

    Array(Vec<Self>),
    NullArray,
}

impl Value {
    pub fn into_string(self) -> Result<String, Self> {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => Ok(s),
            Value::NullString | Value::NullArray | Value::BulkByteString(_) | Value::Array(_) => {
                Err(self)
            }
        }
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
            Value::SimpleString(_) => todo!(),
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
