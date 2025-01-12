#[cfg(test)]
mod tests;

pub use super::deserialize::deserialize_value;
pub use super::serialize::serialize_value;
use anyhow::{anyhow, bail};

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
            _ => bail!(
                "byte: {byte} char: {} is not a valid identifier",
                byte as char
            ),
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

    #[must_use]
    pub fn get_byte_length(&self) -> usize {
        1
    }
}

pub trait GetIdentifier {
    fn get_identifier(&self) -> anyhow::Result<Identifier>;
}

impl GetIdentifier for [u8] {
    fn get_identifier(&self) -> anyhow::Result<Identifier> {
        Identifier::from_byte(*self.first().ok_or(anyhow!("empty slice"))?)
    }
}
