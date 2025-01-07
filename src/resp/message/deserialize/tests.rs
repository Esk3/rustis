use super::*;
use crate::test_helper;

fn f(msg: impl Into<Vec<Value>>) -> anyhow::Result<Message> {
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
    }
}
