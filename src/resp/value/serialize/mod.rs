use super::{super::Value, identifier::Identifier};

#[cfg(test)]
pub(super) mod tests;

#[must_use]
pub fn serialize_value(value: &Value) -> Vec<u8> {
    match value {
        Value::SimpleString(s) => serialize_simple_string(s),
        Value::BulkString(s) => serialize_bulk_string(s),
        Value::BulkByteString(bytes) => serialize_bulk_byte_string(bytes),
        Value::NullString => serialize_null_string().to_vec(),
        Value::Array(arr) => serialize_array(arr),
        Value::NullArray => serialize_null_array().to_vec(),
        Value::Integer(i) => serialize_int(*i),
        Value::SimpleError(s) => serialize_simple_string(s),
    }
}

pub fn serialize_simple_string(s: &str) -> Vec<u8> {
    let (identifier_len, linefeed_len) = (1, 2);
    let mut bytes = Vec::with_capacity(s.len() + identifier_len + linefeed_len);
    bytes.extend_identifier(&Identifier::SimpleString);
    bytes.extend(s.as_bytes());
    bytes.extend_linefeed();
    bytes
}

pub fn serialize_simple_error(s: &str) -> Vec<u8> {
    let (identifier_len, linefeed_len) = (1, 2);
    let mut bytes = Vec::with_capacity(s.len() + identifier_len + linefeed_len);
    bytes.extend_identifier(&Identifier::SimpleError);
    bytes.extend(s.as_bytes());
    bytes.extend_linefeed();
    bytes
}

pub fn serialize_bulk_string(s: &str) -> Vec<u8> {
    let identifier_header_linefeed_padding = 10;
    let mut bytes = Vec::with_capacity(s.len() + identifier_header_linefeed_padding);
    bytes.extend_header(&Identifier::BulkString, s.len().try_into().unwrap());
    bytes.extend(s.as_bytes());
    bytes.extend_linefeed();
    bytes
}

pub fn serialize_bulk_byte_string(s: &[u8]) -> Vec<u8> {
    let identifier_header_linefeed_padding = 10;
    let mut bytes = Vec::with_capacity(s.len() + identifier_header_linefeed_padding);
    bytes.extend_header(&Identifier::BulkString, s.len().try_into().unwrap());
    bytes.extend(s);
    bytes.extend_linefeed();
    bytes
}

pub fn serialize_array(arr: &[Value]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_header(&Identifier::Array, arr.len().try_into().unwrap());
    bytes.extend(arr.iter().flat_map(serialize_value));
    bytes
}

pub const fn serialize_null_string() -> [u8; 5] {
    *b"$-1\r\n"
}

pub const fn serialize_null_array() -> [u8; 5] {
    *b"*-1\r\n"
}

pub fn serialize_int(n: i64) -> Vec<u8> {
    let binding = n.to_string();
    let digits = binding.as_bytes();
    let mut result = Vec::with_capacity(1 + 1 + digits.len() + 2);
    result.extend_identifier(&Identifier::Integer);
    result.extend(digits);
    result.extend_linefeed();
    result
}

trait ExtendLinefeed {
    fn extend_linefeed(&mut self);
}

impl ExtendLinefeed for Vec<u8> {
    fn extend_linefeed(&mut self) {
        self.extend(b"\r\n");
    }
}

trait ExtendIdentifier {
    fn extend_identifier(&mut self, identifier: &Identifier);
}

impl ExtendIdentifier for Vec<u8> {
    fn extend_identifier(&mut self, identifier: &Identifier) {
        self.push(identifier.as_byte());
    }
}

trait ExtendHeader {
    fn extend_header(&mut self, identifier: &Identifier, length: isize);
}

impl ExtendHeader for Vec<u8> {
    fn extend_header(&mut self, identifier: &Identifier, length: isize) {
        self.extend_identifier(identifier);
        self.extend(length.to_string().as_bytes());
        self.extend_linefeed();
    }
}
