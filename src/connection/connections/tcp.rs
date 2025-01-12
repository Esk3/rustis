use std::io::{Read, Write};

use crate::{
    connection::{self, Connection, ConnectionError, ConnectionResult},
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

    fn read_values(&mut self) -> ConnectionResult<Vec<connection::Value>> {
        let bytes_read = match self.stream.read(&mut self.buf[self.i..]) {
            Ok(bytes_read) => bytes_read,
            Err(err) => {
                tracing::debug!("err reading from connection {err}");
                if let std::io::ErrorKind::WouldBlock = err.kind() {
                    return Err(ConnectionError::Io(err));
                }
                todo!();
            }
        };
        if bytes_read == 0 {
            todo!();
        }
        self.i += bytes_read;
        tracing::trace!(
            "read into buffer: {:?}",
            String::from_utf8(self.buf[..self.i].to_vec())
        );
        let mut messages = Vec::new();
        while self.i > 0 {
            let (value, bytes_consumed) = match deserialize_value(&self.buf[..self.i]) {
                Ok(value) => value,
                Err(err) => {
                    tracing::debug!("err deserializing value: {err}");
                    todo!()
                }
            };
            tracing::debug!("deserialized resp:\r\n{value:?}");
            self.buf.rotate_left(bytes_consumed);
            self.i -= bytes_consumed;
            let message = connection::Value::new(value, bytes_consumed);
            messages.push(message);
        }
        Ok(messages)
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        self.stream.peer_addr().unwrap()
    }

    fn write_values(&mut self, values: Vec<resp::Value>) -> ConnectionResult<usize> {
        tracing::debug!("seralizing resp:\r\n{values:?}");
        let bytes = values.iter().flat_map(serialize_value).collect::<Vec<u8>>();
        if let Err(err) = self.stream.write_all(&bytes) {
            tracing::debug!("err wrting to connection {err}");
            todo!();
        }
        if let Err(err) = self.stream.flush() {
            tracing::debug!("err fushing buffer {err}");
            todo!();
        }
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
