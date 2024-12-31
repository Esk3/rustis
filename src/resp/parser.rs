use crate::connection::{ConnectionMessage, Input, Output, ReplConf};

use super::Value;

pub trait Parse {
    fn parse(value: Value) -> anyhow::Result<ConnectionMessage>;
}

pub struct RespParser;
impl Parse for RespParser {
    fn parse(value: Value) -> anyhow::Result<ConnectionMessage> {
        let arr = value.into_array().unwrap();
        if let Ok([cmd]) = TryInto::<[Value; 1]>::try_into(arr) {
            if cmd.eq_ignore_ascii_case("PING") {
                return Ok(ConnectionMessage::Input(Input::Ping));
            }
            if cmd.eq_ignore_ascii_case("REPLCONF") {
                return Ok(ConnectionMessage::Input(Input::ReplConf(
                    ReplConf::ListingPort(1),
                )));
            }
        }
        todo!()
    }
}

pub trait Encode {
    fn encode(message: ConnectionMessage) -> anyhow::Result<Value>;
}

pub struct RespEncoder;
impl Encode for RespEncoder {
    fn encode(message: ConnectionMessage) -> anyhow::Result<Value> {
        let value = match message {
            ConnectionMessage::Input(_) => todo!(),
            ConnectionMessage::Output(output) => match output {
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
                        .map(|value| Self::encode(ConnectionMessage::Output(value)).unwrap())
                        .collect(),
                ),
                Output::Multi => todo!(),
                Output::Queued => Value::SimpleString("Queued".into()),
                Output::MultiError => todo!(),
            },
        };
        Ok(value)
    }
}
