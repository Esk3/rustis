use std::io::{Read, Write};

use anyhow::anyhow;

use crate::{
    connection::{self, Connection, ConnectionResult},
    resp::{
        self,
        value::{deserialize_value, serialize_value},
    },
};

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
        let stream = std::net::TcpStream::connect(addr)?;
        Ok(Self::from(stream))
    }

    fn read_value(&mut self) -> ConnectionResult<connection::Value> {
        let bytes_read = self.stream.read(&mut self.buf[self.i..]).unwrap();
        self.i += bytes_read;
        tracing::debug!(
            "buffer: {:?}",
            String::from_utf8(self.buf[..self.i].to_vec())
        );
        let (value, bytes_consumed) = deserialize_value(&self.buf[..self.i]).unwrap();
        tracing::debug!("got value {value:?}");
        self.buf.rotate_left(bytes_consumed);
        self.i -= bytes_consumed;
        let message = connection::Value::new(value, bytes_consumed);
        tracing::debug!("got message {message:?}");
        Ok(message)
    }

    fn write_value(&mut self, value: resp::Value) -> ConnectionResult<usize> {
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

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        todo!()
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
