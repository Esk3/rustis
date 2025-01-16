use crate::resp::value::identifier::Identifier;

use super::util::{ExtendHeader, ExtendLinefeed};

#[cfg(test)]
mod tests;

#[must_use]
pub fn serialize_bulk_string(s: &str) -> Vec<u8> {
    let identifier_header_linefeed_padding = 10;
    let mut bytes = Vec::with_capacity(s.len() + identifier_header_linefeed_padding);
    bytes.extend_header(&Identifier::BulkString, s.len().try_into().unwrap());
    bytes.extend(s.as_bytes());
    bytes.extend_linefeed();
    bytes
}
