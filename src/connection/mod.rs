use std::{
    fmt::Debug,
    io::{Read, Write},
};

use thiserror::Error;

use crate::resp::{
    parser::{Encode, Parse, RespEncoder, RespParser},
    protocol::{deserialize_value, serialize_value},
};

pub mod handshake;
pub mod incoming;
pub mod outgoing;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub use tests::*;

pub trait Connection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self>
    where
        Self: Sized;
    fn read_message(&mut self) -> ConnectionResult<ConnectionMessage>;
    fn write_message(&mut self, message: ConnectionMessage) -> ConnectionResult<usize>;
}

pub struct RedisTcpConnection {
    stream: std::net::TcpStream,
    buf: [u8; 1024],
    i: usize,
}

impl Connection for RedisTcpConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn read_message(&mut self) -> ConnectionResult<ConnectionMessage> {
        let bytes_read = self.stream.read(&mut self.buf[self.i..]).unwrap();
        self.i += bytes_read;
        tracing::debug!(
            "buffer: {:?}",
            String::from_utf8(self.buf[..self.i].to_vec()).unwrap()
        );
        let (value, bytes_consumed) = deserialize_value(&self.buf[..self.i]).unwrap();
        tracing::debug!("got value {value:?}");
        self.buf.rotate_left(bytes_consumed);
        self.i -= bytes_consumed;
        let message = RespParser::parse(value).unwrap();
        tracing::debug!("got message {message:?}");
        Ok(message)
    }

    fn write_message(&mut self, message: ConnectionMessage) -> ConnectionResult<usize> {
        let value = RespEncoder::encode(message).unwrap();
        tracing::debug!("got value: {value:?}");
        let bytes = serialize_value(&value);
        tracing::debug!(
            "serialized: {} \r\n {:?}",
            String::from_utf8(bytes.clone()).unwrap(),
            bytes
        );
        self.stream.write_all(&bytes).unwrap();
        Ok(bytes.len())
    }
}

impl From<std::net::TcpStream> for RedisTcpConnection {
    fn from(value: std::net::TcpStream) -> Self {
        Self {
            stream: value,
            buf: [0; 1024],
            i: 0,
        }
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("end of input")]
    EndOfInput,
    #[error("{0}")]
    Any(#[from] anyhow::Error),
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
        expiry: Option<std::time::SystemTime>,
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
    Ok,
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
