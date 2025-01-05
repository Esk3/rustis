use crate::resp::{Input, Message, Output, ReplConf};

use super::super::Value;

pub fn deserialize_message(value: Value) -> anyhow::Result<Message> {
    let arr = value.into_array().unwrap();
    if let Ok([cmd]) = TryInto::<[Value; 1]>::try_into(arr) {
        if cmd.eq_ignore_ascii_case("PING") {
            return Ok(Message::Input(Input::Ping));
        }
        if cmd.eq_ignore_ascii_case("REPLCONF") {
            return Ok(Message::Input(Input::ReplConf(ReplConf::ListingPort(1))));
        }
    }
    todo!()
}
