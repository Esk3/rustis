use crate::resp::value::identifier::Identifier;

use super::util::{ExtendIdentifier, ExtendLinefeed as _};

#[cfg(test)]
mod tests;

#[must_use]
pub fn serialize_int(n: i64) -> Vec<u8> {
    let binding = n.to_string();
    let digits = binding.as_bytes();
    let mut result = Vec::with_capacity(1 + 1 + digits.len() + 2);
    result.extend_identifier(&Identifier::Integer);
    result.extend(digits);
    result.extend_linefeed();
    result
}
