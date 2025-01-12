use crate::resp::value::identifier::Identifier;

use super::util::{ExtendHeader, ExtendLinefeed};

#[cfg(test)]
mod tests;

pub fn serialize_bulk_byte_string(s: &[u8]) -> Vec<u8> {
    let identifier_header_linefeed_padding = 10;
    let mut bytes = Vec::with_capacity(s.len() + identifier_header_linefeed_padding);
    bytes.extend_header(&Identifier::BulkString, s.len().try_into().unwrap());
    bytes.extend(s);
    bytes.extend_linefeed();
    bytes
}
