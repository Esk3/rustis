use std::fmt::Debug;

use thiserror::Error;
use tracing::instrument;

pub mod hanlder;
pub mod incoming;
pub mod outgoing;

pub trait Connection {
    fn read_resp(&mut self, buf: &mut [u8]) -> ConnectionResult<usize>;
    fn write_resp(&mut self, buf: &[u8]) -> ConnectionResult<()>;
    fn from_connection<C>(value: C) -> Self;
    fn read_command(&mut self) -> ConnectionResult<ConnectionMessage>;
    fn write_command(&mut self, command: ConnectionMessage) -> ConnectionResult<usize>;
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("end of input")]
    EndOfInput,
}

pub type ConnectionResult<T> = Result<T, ConnectionError>;

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionMessage {
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

    Multi,
    CommitMulti,

    ReplConf(ReplConf),
    Psync,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Pong,
    Get(Option<String>),
    Set,

    Multi,
    Queued,

    ReplConf(ReplConf),
    Psync,
    Null,
    Ok,
    Array(Vec<Self>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReplConf {
    ListingPort(u16),
    Capa(String),
    GetAck(i32),
    Ack(i32),
}
