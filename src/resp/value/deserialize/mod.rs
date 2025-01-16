use crate::resp::Value;
use anyhow::bail;
use array::deserialize_array;
use bulk_string::deserialize_bulk_string;
use simple_string::deserialize_simple_string;
use util::GetHeader;

pub mod array;
pub mod bulk_string;
pub mod integer;
pub mod simple_error;
pub mod simple_string;
pub(super) mod util;

use super::identifier::{GetIdentifier, Identifier};

#[cfg(test)]
mod tests;

pub fn deserialize_value(bytes: &[u8]) -> anyhow::Result<(Value, usize)> {
    let ident = bytes.get_identifier()?;

    let value = match ident {
        Identifier::SimpleString => {
            let (s, length) = deserialize_simple_string(&bytes[ident.get_byte_length()..]).unwrap();
            (Value::SimpleString(s), length + ident.get_byte_length())
        }
        Identifier::SimpleError => todo!(),
        Identifier::Integer => todo!(),
        Identifier::BulkString => {
            let (length, header_length) = bytes.get_header()?;
            let Ok(length) = length.try_into() else {
                if length != -1 {
                    bail!("invalid length in header")
                }
                return Ok((Value::NullString, header_length));
            };
            let (bytes, length) = deserialize_bulk_string(&bytes[header_length..], length).unwrap();
            let value = match String::from_utf8(bytes) {
                Ok(s) => Value::BulkString(s),
                Err(err) => Value::BulkByteString(err.into_bytes()),
            };
            (value, length + header_length)
        }
        Identifier::Array => {
            let (array_size, header_length) = bytes.get_header()?;
            let Ok(array_size) = array_size.try_into() else {
                if array_size != -1 {
                    bail!("invalid length in header")
                }
                return Ok((Value::NullArray, header_length));
            };
            let (arr, array_length) = deserialize_array(&bytes[header_length..], array_size)?;
            (Value::Array(arr), header_length + array_length)
        }
        Identifier::Null => todo!(),
        Identifier::Boolean => todo!(),
        Identifier::Double => todo!(),
        Identifier::BigNumber => todo!(),
        Identifier::BulkError => todo!(),
        Identifier::VerbatimString => todo!(),
        Identifier::Map => todo!(),
        Identifier::Attribute => todo!(),
        Identifier::Set => todo!(),
        Identifier::Pushe => todo!(),
    };
    Ok(value)
}
