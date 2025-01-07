use anyhow::anyhow;

use crate::resp::{Input, Value};

use super::TryDeserializeResult;

#[cfg(test)]
mod tests;

pub fn try_deserialize(arr: Vec<Value>) -> TryDeserializeResult {
    if arr.is_empty() || !arr.first().unwrap().eq_ignore_ascii_case("GET") {
        return TryDeserializeResult::Ignore(arr);
    }
    let Some(Ok(key)) = arr.into_iter().nth(1).map(Value::expect_string) else {
        return TryDeserializeResult::Err(anyhow!("key missing"));
    };
    Input::Get(key).into()
}
