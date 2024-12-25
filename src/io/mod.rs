use std::usize;

use crate::resp::Value;

pub trait Io {
    fn read_value(&mut self) -> anyhow::Result<Value>;
    fn write_value(&mut self, value: Value) -> anyhow::Result<usize>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkMessage {
    Input(Input),
    Output(Output),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Ping,

    Get(String),
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
        get: bool,
    },

    ReplConf,
    Psync,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Pong,
    Get(Option<String>),
    Set,

    ReplConf,
    Psync,
}
