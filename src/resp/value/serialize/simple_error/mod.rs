use crate::resp::value::identifier::Identifier;

use super::util::{ExtendIdentifier, ExtendLinefeed};

#[cfg(test)]
mod tests;

pub fn serialize_simple_error(s: &str) -> Vec<u8> {
    let (identifier_len, linefeed_len) = (1, 2);
    let mut bytes = Vec::with_capacity(s.len() + identifier_len + linefeed_len);
    bytes.extend_identifier(&Identifier::SimpleError);
    bytes.extend(s.as_bytes());
    bytes.extend_linefeed();
    bytes
}
