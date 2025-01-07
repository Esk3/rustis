use std::io::{Read, Write};

use crate::{
    connection::{Connection, ConnectionResult},
    listner::RedisListner,
    resp::{
        self,
        message::{deserialize::deserialize_message, serialize::serialize_message},
        value::{deserialize_value, serialize_value},
    },
};

#[derive(Debug)]
pub struct RedisStdInOutConnection {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    buf: [u8; 1024],
    i: usize,
}

impl RedisStdInOutConnection {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            buf: [0; 1024],
            i: 0,
        }
    }
}

impl Connection for RedisStdInOutConnection {
    fn connect(_addr: std::net::SocketAddr) -> ConnectionResult<Self>
    where
        Self: Sized,
    {
        Ok(Self::new())
    }

    fn read_message(&mut self) -> ConnectionResult<resp::Message> {
        let bytes_read = self.stdin.read(&mut self.buf[self.i..])?;
        self.i += bytes_read;
        tracing::debug!(
            "buffer: {:?}",
            String::from_utf8(self.buf[..self.i].to_vec())
        );
        let (value, bytes_consumed) = deserialize_value(&self.buf[..self.i])?;
        tracing::debug!("got value {value:?}");
        self.buf.rotate_left(bytes_consumed);
        self.i -= bytes_consumed;
        let message = deserialize_message(value)?;
        tracing::debug!("got message {message:?}");
        Ok(message)
    }

    fn write_message(&mut self, message: resp::Message) -> ConnectionResult<usize> {
        let value = serialize_message(message).unwrap();
        tracing::debug!("got value: {value:?}");
        let bytes = serialize_value(&value);
        tracing::debug!(
            "serialized: {} \r\n {:?}",
            String::from_utf8(bytes.clone()).unwrap(),
            bytes
        );
        self.stdout.write_all(&bytes).unwrap();
        Ok(bytes.len())
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        todo!()
    }
}

impl RedisListner for RedisStdInOutConnection {
    type Connection = Self;

    fn get_port(&self) -> u16 {
        0
    }

    fn bind(port: u16) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn incoming(self) -> impl Iterator<Item = Self::Connection> {
        std::iter::once(Self::new())
    }
}
