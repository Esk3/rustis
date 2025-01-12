use super::super::Value;

pub mod array;
pub mod bulk_byte_string;
pub mod bulk_string;
pub mod integer;
pub mod null_array;
pub mod null_string;
pub mod simple_error;
pub mod simple_string;
mod util;

#[cfg(test)]
pub(super) mod tests;

#[must_use]
pub fn serialize_value(value: &Value) -> Vec<u8> {
    match value {
        Value::SimpleString(s) => simple_string::serialize_simple_string(s),
        Value::BulkString(s) => bulk_string::serialize_bulk_string(s),
        Value::BulkByteString(bytes) => bulk_byte_string::serialize_bulk_byte_string(bytes),
        Value::NullString => null_string::serialize_null_string().to_vec(),
        Value::Array(arr) => array::serialize_array(arr),
        Value::NullArray => null_array::serialize_null_array().to_vec(),
        Value::Integer(i) => integer::serialize_int(*i),
        Value::SimpleError(s) => simple_error::serialize_simple_error(s),
        Value::Raw(raw) => raw.clone(),
    }
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for Value {
    fn serialize(&self) -> Vec<u8> {
        serialize_value(self)
    }
}
