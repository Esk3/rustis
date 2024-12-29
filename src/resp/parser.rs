use crate::io::NetworkMessage;

use super::Value;

pub trait Parse {
    fn parse(value: Value) -> anyhow::Result<NetworkMessage>;
}

pub struct RespParser;
impl Parse for RespParser {
    fn parse(value: Value) -> anyhow::Result<NetworkMessage> {
        let arr = value.into_array().unwrap();
        if let Ok([cmd]) = TryInto::<[Value; 1]>::try_into(arr) {
            if cmd.eq_ignore_ascii_case("PING") {
                return Ok(NetworkMessage::Input(crate::io::Input::Ping));
            }
            if cmd.eq_ignore_ascii_case("REPLCONF") {
                return Ok(NetworkMessage::Input(crate::io::Input::ReplConf(
                    crate::io::ReplConf::ListingPort(1),
                )));
            }
        }
        todo!()
    }
}

pub trait Encode {
    fn encode(message: NetworkMessage) -> anyhow::Result<Value>;
}

pub struct RespEncoder;
impl Encode for RespEncoder {
    fn encode(message: NetworkMessage) -> anyhow::Result<Value> {
        let value = match message {
            NetworkMessage::Input(_) => todo!(),
            NetworkMessage::Output(output) => match output {
                crate::io::Output::Pong => Value::SimpleString("PONG".into()),
                crate::io::Output::Get(value) => {
                    if let Some(value) = value {
                        Value::BulkString(value)
                    } else {
                        Value::NullArray
                    }
                }
                crate::io::Output::Set => Value::SimpleString("Ok".into()),
                crate::io::Output::ReplConf(_) => Value::SimpleString("Ok".into()),
                crate::io::Output::Psync => todo!(),
                crate::io::Output::Null => Value::NullArray,
                crate::io::Output::Ok => Value::SimpleString("Ok".into()),
                crate::io::Output::Array(arr) => Value::Array(
                    arr.into_iter()
                        .map(|value| Self::encode(NetworkMessage::Output(value)).unwrap())
                        .collect(),
                ),
                crate::io::Output::Multi => todo!(),
                crate::io::Output::Queued => Value::SimpleString("Queued".into()),
            },
        };
        Ok(value)
    }
}
