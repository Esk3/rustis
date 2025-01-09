use anyhow::anyhow;

use crate::resp::{Input, Value};

use super::TryDeserializeResult;

//#[cfg(test)]
//mod tests;

pub fn try_deserialize(arr: Vec<Value>) -> TryDeserializeResult {
    if arr.is_empty() || !arr.first().unwrap().eq_ignore_ascii_case("CLIENT") {
        return TryDeserializeResult::Ignore(arr);
    }
    Input::Client.into()
}
