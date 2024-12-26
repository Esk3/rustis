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
                crate::io::Output::Get(_) => todo!(),
                crate::io::Output::Set => todo!(),
                crate::io::Output::ReplConf => todo!(),
                crate::io::Output::Psync => todo!(),
                crate::io::Output::Null => todo!(),
                crate::io::Output::Ok => todo!(),
                crate::io::Output::Array(_) => todo!(),
            },
        };
        Ok(value)
    }
}
