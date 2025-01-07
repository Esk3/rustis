use std::{
    fmt::Debug,
    io::{Read, Write},
};

use thiserror::Error;

use crate::resp::{
    self,
    message::{deserialize::deserialize_message, serialize::serialize_message},
    value::{deserialize_value, serialize_value},
};

pub mod connections;
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
    fn read_message(&mut self) -> ConnectionResult<resp::Message>;
    fn write_message(&mut self, message: resp::Message) -> ConnectionResult<usize>;
    fn get_peer_addr(&self) -> std::net::SocketAddr;
}
#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("end of input")]
    EndOfInput,
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

pub type ConnectionResult<T> = Result<T, ConnectionError>;
