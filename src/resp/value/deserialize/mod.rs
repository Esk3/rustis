use crate::resp::Value;
use anyhow::{anyhow, bail};

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

pub fn deserialize_simple_string(bytes: &[u8]) -> anyhow::Result<(String, usize)> {
    let linefeed = bytes.find_linefeed().unwrap().unwrap();
    Ok((
        String::from_utf8(bytes[..linefeed].to_vec()).unwrap(),
        linefeed + 2,
    ))
}

pub fn deserialize_header(mut bytes: &[u8]) -> anyhow::Result<(isize, usize)> {
    let mut length = 0;
    if bytes
        .first()
        .is_some_and(|byte| Identifier::from_byte(*byte).is_ok())
    {
        bytes = &bytes[1..];
        length += 1;
    }
    let linefeed = bytes
        .find_linefeed()?
        .ok_or(anyhow!("linefeed not found"))?;
    length += linefeed + 2;
    let digits = &bytes[..linefeed];
    let digits = String::from_utf8(digits.to_vec())?;
    let number = digits.parse()?;
    Ok((number, length))
}

pub fn deserialize_bulk_string(bytes: &[u8], length: usize) -> anyhow::Result<(Vec<u8>, usize)> {
    assert!(&bytes[length..].is_at_linefeed().unwrap());
    let s = bytes[..length].to_vec();
    Ok((s, length + 2))
}

pub fn deserialize_array(mut bytes: &[u8], items: usize) -> anyhow::Result<(Vec<Value>, usize)> {
    let mut result = Vec::with_capacity(items);
    let mut length = 0;
    for _ in 0..items {
        let (value, bytes_consumed) = deserialize_value(bytes)?;
        result.push(value);
        length += bytes_consumed;
        bytes = &bytes[bytes_consumed..];
    }
    Ok((result, length))
}

fn is_linefeed(cr: u8, lf: u8) -> anyhow::Result<bool> {
    if cr == b'\n' {
        bail!("found newline before cr");
    }
    if cr == b'\r' {
        if lf != b'\n' {
            bail!("expected newline found: byte [{cr}] char: [{}]", cr as char);
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

trait FindLinefeed {
    fn find_linefeed(&self) -> anyhow::Result<Option<usize>>;
    fn is_at_linefeed(&self) -> anyhow::Result<bool>;
}

impl FindLinefeed for [u8] {
    fn find_linefeed(&self) -> anyhow::Result<Option<usize>> {
        for (i, win) in self.windows(2).enumerate() {
            let (cr, lf) = (win[0], win[1]);
            let is_linefeed = is_linefeed(cr, lf)?;
            if is_linefeed {
                return Ok(Some(i));
            }
        }
        if [b'\r', b'\n'].map(Some).contains(&self.last().copied()) {
            bail!("found single linefeed or cr");
        }

        Ok(None)
    }

    fn is_at_linefeed(&self) -> anyhow::Result<bool> {
        let (Some(cr), Some(lf)) = (self.first(), self.get(1)) else {
            return Ok(false);
        };
        is_linefeed(*cr, *lf)
    }
}

pub trait GetHeader {
    fn get_header(&self) -> anyhow::Result<(isize, usize)>;
}
impl GetHeader for [u8] {
    fn get_header(&self) -> anyhow::Result<(isize, usize)> {
        deserialize_header(self)
    }
}
