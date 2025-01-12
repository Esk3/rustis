use std::fmt::Debug;
use thiserror::Error;

use crate::resp::{self};

pub mod handshake;
pub mod incoming;
pub mod outgoing;
pub mod stream;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

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
