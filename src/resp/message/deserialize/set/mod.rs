use crate::resp::{value::Value, Input};

use super::TryDeserializeResult;

#[cfg(test)]
mod tests;

#[must_use]
pub fn try_deserialize(arr: Vec<Value>) -> TryDeserializeResult {
    let Some(first) = arr.first() else {
        return TryDeserializeResult::Ignore(arr);
    };
    if !first.eq_ignore_ascii_case("SET") {
        return TryDeserializeResult::Ignore(arr);
    }
    let mut iter = arr.into_iter().skip(1);
    let Some(Ok(key)) = iter.next().map(Value::expect_string) else {
        todo!()
    };
    let Some(Ok(value)) = iter.next().map(Value::expect_string) else {
        todo!()
    };
    TryDeserializeResult::Ok(
        Input::Set {
            key,
            value,
            expiry: None,
            get: false,
        }
        .into(),
    )
}
