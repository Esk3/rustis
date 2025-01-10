use std::fmt::Debug;

use thiserror::Error;

use crate::resp::{self};

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
    fn read_value(&mut self) -> ConnectionResult<Value>;
    fn write_value(&mut self, message: resp::Value) -> ConnectionResult<usize>;
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

#[derive(Debug)]
pub struct Value {
    pub value: resp::Value,
    pub bytes_read: usize,
}

impl Value {
    fn new(value: resp::Value, bytes_consumed: usize) -> Self {
        Self {
            value,
            bytes_read: bytes_consumed,
        }
    }
}
