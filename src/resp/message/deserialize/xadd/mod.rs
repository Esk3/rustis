use anyhow::anyhow;

use crate::resp::{Input, Value};

use super::TryDeserializeResult;

#[cfg(test)]
mod tests;

pub fn try_deserialize(arr: Vec<Value>) -> TryDeserializeResult {
    if arr.is_empty() || !arr.first().unwrap().eq_ignore_ascii_case("XADD") {
        return TryDeserializeResult::Ignore(arr);
    }
    let mut iter = arr.into_iter().skip(1);
    let Some(Ok(stream_key)) = iter.next().map(Value::expect_string) else {
        return TryDeserializeResult::Err(anyhow!("key missing"));
    };
    let Some(Ok(value)) = iter.next().map(Value::expect_string) else {
        todo!();
    };
    TryDeserializeResult::Ok(
        Input::XAdd {
            stream_key,
            entry_id: None,
            value,
        }
        .into(),
    )
}
