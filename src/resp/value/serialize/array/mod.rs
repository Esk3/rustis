use crate::resp::{value::identifier::Identifier, Value};

use super::util::ExtendHeader;

#[cfg(test)]
mod tests;

pub fn serialize_array(arr: &[Value]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_header(&Identifier::Array, arr.len().try_into().unwrap());
    bytes.extend(arr.iter().flat_map(super::serialize_value));
    bytes
}
