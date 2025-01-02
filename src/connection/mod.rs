use std::fmt::Debug;

use thiserror::Error;

pub mod hanlder;
pub mod incoming;
pub mod outgoing;

pub trait Connection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self>
    where
        Self: Sized;
    fn read_message(&mut self) -> ConnectionResult<ConnectionMessage>;
    fn write_message(&mut self, command: ConnectionMessage) -> ConnectionResult<usize>;
}

pub struct RedisTcpConnection(std::net::TcpStream);

impl Connection for RedisTcpConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn read_message(&mut self) -> ConnectionResult<ConnectionMessage> {
        Ok(ConnectionMessage::Input(Input::Ping))
    }

    fn write_message(&mut self, command: ConnectionMessage) -> ConnectionResult<usize> {
        Ok(1)
    }
}

impl From<std::net::TcpStream> for RedisTcpConnection {
    fn from(value: std::net::TcpStream) -> Self {
        Self(value)
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("end of input")]
    EndOfInput,
}

pub type ConnectionResult<T> = Result<T, ConnectionError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConnectionMessage {
    Input(Input),
    Output(Output),
}

impl ConnectionMessage {
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Input {
    Ping,

    Get(String),
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
        get: bool,
    },

    Multi,
    CommitMulti,

    ReplConf(ReplConf),
    Psync,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Output {
    Pong,
    Get(Option<String>),
    Set,

    Multi,
    MultiError,
    Queued,

    ReplConf(ReplConf),
    Psync,
    Null,
    Ok,
    Array(Vec<Self>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReplConf {
    ListingPort(u16),
    Capa(String),
    GetAck(i32),
    Ack(i32),
}

impl From<Output> for ConnectionMessage {
    fn from(value: Output) -> Self {
        Self::Output(value)
    }
}
impl From<Input> for ConnectionMessage {
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
