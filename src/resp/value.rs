#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    SimpleString(String),
    BulkString(String),
    BulkByteString(Vec<u8>),

    Array(Vec<Self>),
}

impl Value {
    pub fn into_string(self) -> Result<String, Self> {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => Ok(s),
            Value::BulkByteString(_) | Value::Array(_) => Err(self),
        }
    }
    pub fn into_array(self) -> Result<Vec<Self>, Self> {
        if let Self::Array(arr) = self {
            Ok(arr)
        } else {
            Err(self)
        }
    }
    #[must_use]
    pub fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => s.eq_ignore_ascii_case(other),
            Value::BulkByteString(_) | Value::Array(_) => false,
        }
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Value::SimpleString(s) | Value::BulkString(s) => other == s,
            Value::BulkByteString(_) | Value::Array(_) => false,
        }
    }
}
