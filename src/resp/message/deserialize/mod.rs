use anyhow::bail;

use crate::resp::{Message, Output};

use super::super::Value;

pub mod client;
pub mod get;
pub mod len;
pub mod set;
mod try_deserialize_result;
pub mod xadd;
pub mod xrange;
pub use try_deserialize_result::TryDeserializeResult;

#[cfg(test)]
pub mod tests;

pub fn deserialize_message(value: Value) -> anyhow::Result<Message> {
    let arr = value.into_array().unwrap();
    let arr = match try_deserialize_variable_length(arr) {
        TryDeserializeResult::Ok(message) => return Ok(message),
        TryDeserializeResult::Err(err) => return Err(err),
        TryDeserializeResult::Ignore(arr) => arr,
    };
    match arr.len() {
        1 => len::deserialize_len_one(arr.try_into().expect("length is one")),
        2 => len::deserialize_len_two(arr.try_into().expect("length is two")),
        _ => bail!("command not found, {arr:?}"),
    }
}

pub fn try_deserialize_get_response(value: Value) -> anyhow::Result<Output> {
    Ok(Output::Get(None))
}

fn try_deserialize_variable_length(arr: Vec<Value>) -> TryDeserializeResult {
    TryDeserializeResult::new(arr)
        .try_next(set::try_deserialize)
        .try_next(get::try_deserialize)
        .try_next(xadd::try_deserialize)
        .try_next(xrange::try_deserialize)
        .try_next(client::try_deserialize)
}
