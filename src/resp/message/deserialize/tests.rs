use super::*;
use crate::{resp::Input, test_helper};

pub fn f(msg: impl Into<Vec<Value>>) -> anyhow::Result<Message> {
    deserialize_message(Value::Array(msg.into()))
}

test_helper! {
    Test {}
    [eq Input::Ping.into()]
    ping() {
        f([
            Value::SimpleString("PING".into())
        ]).unwrap()
    };
    [eq Output::Pong.into()]
    pong() {
        f([
            Value::SimpleString("PONG".into())
        ]).unwrap()
    };
    [eq Input::Get("MyKey".into()).into()]
    get_input() {
        f([
            Value::BulkString("GET".into()),
            Value::BulkString("MyKey".into())
        ]).unwrap()
    };
    [eq Output::Get(None)]
    get_output() {
        try_deserialize_get_response(Value::NullArray).unwrap()
    };
    [eq Input::Multi.into()]
    multi() {
        f([Value::BulkString("MULTI".into())]).unwrap()
    };
    [eq Input::CommitMulti.into()]
    exec_multi() {
        f([Value::BulkString("EXEC".into())]).unwrap()
    }
}

#[test]
fn returns_unknown_command_error_on_unknown_command() {
    let messages = [
        [Value::BulkString("Do not Find".into())].to_vec(),
        std::iter::repeat(Value::BulkString("abc".into()))
            .take(2)
            .collect(),
        std::iter::repeat(Value::BulkString("abc".into()))
            .take(3)
            .collect(),
        std::iter::repeat(Value::BulkString("abc".into()))
            .take(4)
            .collect(),
    ];
    for message in messages {
        assert!(f(message).is_err());
    }
}
