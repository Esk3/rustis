use crate::resp::Value;
use anyhow::bail;
use array::deserialize_array;
use bulk_string::deserialize_bulk_string;
use info::MapValue;
use simple_string::deserialize_simple_string;
use util::GetHeader;

pub use deserializer::Deserializer;

pub mod array;
pub mod bulk_string;
mod deserializer;
mod info;
pub mod integer;
pub mod simple_error;
pub mod simple_string;
pub mod util;

use super::{
    identifier::{GetIdentifier, Identifier},
    IntoRespArray,
};

#[cfg(test)]
mod tests;

pub fn deserialize_value(bytes: &[u8]) -> anyhow::Result<(Value, usize)> {
    let mut deserializer = Deserializer::new(bytes, 0);
    let ident = deserializer
        .advance(|bytes| {
            let ident = bytes.get_identifier()?;
            let size = ident.get_byte_length();
            Ok(info::DeserializeInfo::new(ident, size))
        })?
        .value;

    let value = match ident {
        Identifier::SimpleString => deserializer
            .deserialize(deserialize_simple_string)?
            .map_value(Value::SimpleString),
        Identifier::SimpleError => todo!(),
        Identifier::Integer => todo!(),
        Identifier::BulkString => {
            deserializer.deserialize_header(Value::NullString, |bytes, length| {
                deserialize_bulk_string(bytes, length).map_value(|bytes| {
                    match String::from_utf8(bytes) {
                        Ok(s) => Value::BulkString(s),
                        Err(err) => Value::BulkByteString(err.into_bytes()),
                    }
                })
            })?
        }
        Identifier::Array => deserializer
            .deserialize_header(Value::NullArray, |bytes, length| {
                deserialize_array(bytes, length).map_value(|value| value.into_array())
            })?,
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
    Ok(value.into())
}
