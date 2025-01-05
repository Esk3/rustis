use crate::resp::{Input, Message, Output, ReplConf};

use super::super::Value;

pub fn serialize_message(message: Message) -> anyhow::Result<Value> {
    let value = match message {
        Message::Input(_) => todo!(),
        Message::Output(output) => match output {
            Output::Pong => Value::SimpleString("PONG".into()),
            Output::Get(value) => {
                if let Some(value) = value {
                    Value::BulkString(value)
                } else {
                    Value::NullArray
                }
            }
            Output::Set => Value::SimpleString("Ok".into()),
            Output::ReplConf(_) => Value::SimpleString("Ok".into()),
            Output::Psync => todo!(),
            Output::Null => Value::NullArray,
            Output::Ok => Value::SimpleString("Ok".into()),
            Output::Array(arr) => Value::Array(
                arr.into_iter()
                    .map(|value| serialize_message(Message::Output(value)).unwrap())
                    .collect(),
            ),
            Output::Multi => todo!(),
            Output::Queued => Value::SimpleString("Queued".into()),
            Output::MultiError => todo!(),
        },
    };
    Ok(value)
}
