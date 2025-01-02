use anyhow::{anyhow, bail};

use super::Value;

#[cfg(test)]
mod tests;

#[must_use]
pub fn serialize_value(value: &Value) -> Vec<u8> {
    let s = value.clone().into_string().unwrap();
    let mut buf = Vec::with_capacity(s.len());
    buf.push(b'+');
    buf.extend(s.as_bytes());
    buf.extend(b"\r\n");
    buf
}

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
            let length = length.try_into().expect("todo null value");
            let (bytes, length) = deserialize_bulk_string(&bytes[header_length..], length).unwrap();
            (
                Value::BulkString(String::from_utf8(bytes).unwrap()),
                length + header_length,
            )
        }
        Identifier::Array => {
            let (array_size, header_length) = bytes.get_header()?;
            let array_size = array_size.try_into().expect("todo null array");
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
        .find_linefeed()
        .map_err(|_| anyhow!("found invalid token"))?
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

fn is_linefeed(cr: u8, lf: u8) -> Result<bool, ()> {
    if cr == b'\n' {
        return Err(());
    }
    if cr == b'\r' {
        if lf != b'\n' {
            return Err(());
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Identifier {
    SimpleString,
    SimpleError,
    Integer,
    BulkString,
    Array,
    Null,
    Boolean,
    Double,
    BigNumber,
    BulkError,
    VerbatimString,
    Map,
    Attribute,
    Set,
    Pushe,
}

impl Identifier {
    pub fn from_byte(byte: u8) -> anyhow::Result<Self> {
        let ident = match byte {
            b'+' => Self::SimpleString,
            b'-' => Self::SimpleError,
            b':' => Self::Integer,
            b'$' => Self::BulkString,
            b'*' => Self::Array,
            b'_' => Self::Null,
            b'#' => Self::Boolean,
            b',' => Self::Double,
            b'(' => Self::BigNumber,
            b'!' => Self::BulkError,
            b'=' => Self::VerbatimString,
            b'%' => Self::Map,
            b'`' => Self::Attribute,
            b'~' => Self::Set,
            b'>' => Self::Pushe,
            _ => bail!("{byte} is not a valid identifier"),
        };
        Ok(ident)
    }

    #[must_use]
    pub fn as_byte(&self) -> u8 {
        match self {
            Self::SimpleString => b'+',
            Self::SimpleError => b'-',
            Self::Integer => b':',
            Self::BulkString => b'$',
            Self::Array => b'*',
            Self::Null => b'_',
            Self::Boolean => b'#',
            Self::Double => b',',
            Self::BigNumber => b'(',
            Self::BulkError => b'!',
            Self::VerbatimString => b'=',
            Self::Map => b'%',
            Self::Attribute => b'`',
            Self::Set => b'~',
            Self::Pushe => b'>',
        }
    }

    fn get_byte_length(&self) -> usize {
        1
    }
}

trait GetIdentifier {
    fn get_identifier(&self) -> anyhow::Result<Identifier>;
}

impl GetIdentifier for [u8] {
    fn get_identifier(&self) -> anyhow::Result<Identifier> {
        Identifier::from_byte(*self.first().ok_or(anyhow!("empty slice"))?)
    }
}

trait FindLinefeed {
    fn find_linefeed(&self) -> Result<Option<usize>, ()>;
    fn is_at_linefeed(&self) -> Result<bool, ()>;
}

impl FindLinefeed for [u8] {
    fn find_linefeed(&self) -> Result<Option<usize>, ()> {
        for (i, win) in self.windows(2).enumerate() {
            let (cr, lf) = (win[0], win[1]);
            let is_linefeed = is_linefeed(cr, lf)?;
            if is_linefeed {
                return Ok(Some(i));
            }
        }
        if [b'\r', b'\n'].map(Some).contains(&self.last().copied()) {
            Err(())
        } else {
            Ok(None)
        }
    }

    fn is_at_linefeed(&self) -> Result<bool, ()> {
        let (Some(cr), Some(lf)) = (self.first(), self.get(1)) else {
            return Ok(false);
        };
        is_linefeed(*cr, *lf)
    }
}

trait GetHeader {
    fn get_header(&self) -> anyhow::Result<(isize, usize)>;
}
impl GetHeader for [u8] {
    fn get_header(&self) -> anyhow::Result<(isize, usize)> {
        deserialize_header(self)
    }
}
