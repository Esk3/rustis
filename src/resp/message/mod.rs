use anyhow::anyhow;

use crate::repository::stream_repo::stream::EntryId;

pub mod deserialize;
pub mod serialize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    Input(Input),
    Output(Output),
}

impl Message {
    pub fn into_input(self) -> Result<Input, Self> {
        if let Self::Input(input) = self {
            Ok(input)
        } else {
            Err(self)
        }
    }
    pub fn into_output(self) -> Result<Output, Self> {
        if let Self::Output(output) = self {
            Ok(output)
        } else {
            Err(self)
        }
    }

    pub fn expect_input(self) -> anyhow::Result<Input> {
        self.into_input()
            .map_err(|err| anyhow!("expected input got output: {err:?}"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Input {
    Ping,

    Get(String),
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::SystemTime>,
        get: bool,
    },

    Multi,
    CommitMulti,

    ReplConf(ReplConf),
    Psync,

    XAdd {
        stream_key: String,
        entry_id: Option<String>,
        value: String,
    },
    XRead,
    XRange {
        stream_key: String,
        start: EntryId,
        end: EntryId,
    },

    Client,
    Config(Vec<String>),
    Info,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Output {
    Pong,
    Get(Option<String>),
    Set,
    SetGet(Option<String>),

    Multi,
    MultiError,
    Queued,

    ReplConf(ReplConf),
    Psync,
    Null,
    Ok,
    SimpleString(String),
    Array(Vec<Self>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReplConf {
    ListingPort(u16),
    Capa(String),
    GetAck(i32),
    Ack(i32),
    Ok,
}

impl From<Output> for Message {
    fn from(value: Output) -> Self {
        Self::Output(value)
    }
}
impl From<Input> for Message {
    fn from(value: Input) -> Self {
        Self::Input(value)
    }
}
impl From<ReplConf> for Input {
    fn from(value: ReplConf) -> Self {
        Self::ReplConf(value)
    }
}
impl From<ReplConf> for Output {
    fn from(value: ReplConf) -> Self {
        Self::ReplConf(value)
    }
}
