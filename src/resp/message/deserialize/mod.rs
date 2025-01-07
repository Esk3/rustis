use crate::resp::{Input, Message, Output};

use super::super::Value;

#[cfg(test)]
mod tests;

pub fn deserialize_message(value: Value) -> anyhow::Result<Message> {
    let arr = value.into_array().unwrap();
    let arr = match try_deserialize_variable_length(arr)? {
        Ok(message) => return Ok(message),
        Err(value) => value,
    };
    match arr.len() {
        1 => deserialize_len_one(arr.try_into().expect("length is one")),
        2 => deserialize_len_two(arr.try_into().expect("length is two")),
        _ => todo!(),
    }
}

pub fn try_deserialize_get_response(value: Value) -> anyhow::Result<Output> {
    Ok(Output::Get(None))
}

fn try_deserialize_variable_length(arr: Vec<Value>) -> anyhow::Result<Result<Message, Vec<Value>>> {
    Ok(Err(arr))
}

fn deserialize_len_one([value]: [Value; 1]) -> anyhow::Result<Message> {
    let message = match value {
        Value::SimpleString(s) => {
            if s.eq_ignore_ascii_case("PING") {
                Input::Ping.into()
            } else if s.eq_ignore_ascii_case("PONG") {
                Output::Pong.into()
            } else {
                todo!()
            }
        }
        Value::BulkString(s) => {
            if s.eq_ignore_ascii_case("PING") {
                Input::Ping.into()
            } else {
                todo!()
            }
        }
        Value::BulkByteString(_) => todo!(),
        Value::NullString => todo!(),
        Value::Array(_) => todo!(),
        Value::NullArray => todo!(),
    };
    Ok(message)
}

fn deserialize_len_two([first, second]: [Value; 2]) -> Result<Message, anyhow::Error> {
    if first.eq_ignore_ascii_case("GET") {
        Ok(Input::Get(second.into_string().unwrap()).into())
    } else {
        todo!()
    }
}
