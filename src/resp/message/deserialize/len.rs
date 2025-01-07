use anyhow::bail;

use crate::resp::{Input, Message, Output};

use super::super::super::Value;

pub fn deserialize_len_one([value]: [Value; 1]) -> anyhow::Result<Message> {
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
            } else if s.eq_ignore_ascii_case("MULTI") {
                Input::Multi.into()
            } else if s.eq_ignore_ascii_case("EXEC") {
                Input::CommitMulti.into()
            } else {
                bail!("command not found: {s}")
            }
        }
        Value::BulkByteString(_) => todo!(),
        Value::NullString => todo!(),
        Value::Array(_) => todo!(),
        Value::NullArray => todo!(),
    };
    Ok(message)
}

pub fn deserialize_len_two([first, second]: [Value; 2]) -> Result<Message, anyhow::Error> {
    if first.eq_ignore_ascii_case("GET") {
        Ok(Input::Get(second.into_string().unwrap()).into())
    } else {
        bail!("command not found: [{first:?}, {second:?}]");
    }
}
