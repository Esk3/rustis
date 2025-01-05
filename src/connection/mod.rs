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

    fn read_message(&mut self) -> ConnectionResult<resp::Message> {
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
        let message = resp::message::deserialize::deserialize_message(value).unwrap();
        tracing::debug!("got message {message:?}");
        Ok(message)
    }

    fn write_message(&mut self, message: resp::Message) -> ConnectionResult<usize> {
        let value = resp::message::serialize::serialize_message(message).unwrap();
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
