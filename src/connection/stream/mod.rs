use std::fmt::Debug;

use crate::{
    resp::{
        self,
        value::{deserialize_value, serialize_value},
    },
    Message,
};

#[cfg(test)]
mod tests;

pub trait Stream: std::io::Read + std::io::Write {
    type Addr;
    fn connect(addr: Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn peer_addr(&self) -> Self::Addr;
}

pub struct TcpStream(std::net::TcpStream);

impl TcpStream {
    pub fn new(stream: std::net::TcpStream) -> Self {
        Self(stream)
    }
}

impl From<std::net::TcpStream> for TcpStream {
    fn from(value: std::net::TcpStream) -> Self {
        Self::new(value)
    }
}

impl std::io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl std::io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl Stream for TcpStream {
    type Addr = std::net::SocketAddrV4;

    fn connect(addr: Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self(std::net::TcpStream::connect(addr)?))
    }

    fn peer_addr(&self) -> Self::Addr {
        let std::net::SocketAddr::V4(addr) = self.0.peer_addr().unwrap() else {
            unreachable!()
        };
        addr
    }
}

#[derive(Debug)]
pub struct RedisConnection<S> {
    stream: S,
    buf: [u8; 1024],
    i: usize,
}

impl<S> RedisConnection<S>
where
    S: Stream,
{
    pub fn read(&mut self) -> anyhow::Result<Message<resp::Value>> {
        if self.i == 0 {
            let bytes_read = self.stream.read(&mut self.buf).unwrap();
            self.i = bytes_read;
        }
        let (value, bytes_consumed) = deserialize_value(&self.buf).unwrap();
        self.buf.rotate_left(bytes_consumed);
        self.i -= bytes_consumed;
        tracing::trace!("read value: [{value:?}]");
        Ok(Message::new(value, bytes_consumed))
    }

    pub fn read_all(&mut self) -> anyhow::Result<Vec<Message<resp::Value>>> {
        let mut values = Vec::new();
        for i in 0..10 {
            let value = self.read().unwrap();
            values.push(value);
            assert!(i != 8);
            if self.i == 0 {
                break;
            }
        }
        Ok(values)
    }

    pub fn write(&mut self, value: &resp::Value) -> anyhow::Result<usize> {
        let bytes = serialize_value(value);
        self.stream.write_all(&bytes).unwrap();
        self.stream.flush().unwrap();
        tracing::trace!("value written: [{value:?}]");
        Ok(bytes.len())
    }

    pub fn write_all(&mut self, values: &[resp::Value]) -> anyhow::Result<usize> {
        let bytes = values.iter().flat_map(serialize_value).collect::<Vec<u8>>();
        self.stream.write_all(&bytes).unwrap();
        Ok(bytes.len())
    }

    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buf: [0; 1024],
            i: 0,
        }
    }

    pub fn inner(&mut self) -> &mut S {
        &mut self.stream
    }

    pub fn into_inner(self) -> S {
        self.stream
    }
}

#[derive(Debug)]
pub struct PipelineBuffer<S> {
    connection: RedisConnection<S>,
    read_buffer: Vec<Message<resp::Value>>,
    write_buffer: Vec<u8>,
}

impl<S> PipelineBuffer<S>
where
    S: Stream,
{
    pub fn new(stream: S) -> Self {
        Self {
            connection: RedisConnection::new(stream),
            read_buffer: Vec::new(),
            write_buffer: Vec::new(),
        }
    }

    pub fn read(&mut self) -> anyhow::Result<Message<resp::Value>> {
        if let Some(value) = self.read_buffer.pop() {
            tracing::trace!("value read from buffer: [{value:?}]");
            Ok(value)
        } else {
            self.read_buffer
                .extend(self.connection.read_all().unwrap().into_iter().rev());
            tracing::trace!("values read into buffer: {:?}", self.read_buffer);
            tracing::trace!("value read from buffer: [{:?}]", self.read_buffer.last());
            Ok(self.read_buffer.pop().unwrap())
        }
    }

    pub fn write(&mut self, value: &resp::Value) -> anyhow::Result<usize> {
        let value = serialize_value(value);
        let len = value.len();
        tracing::trace!("value written to buffer: [{value:?}]");
        self.write_buffer.extend(value);
        if self.read_buffer.is_empty() {
            self.connection
                .inner()
                .write_all(&self.write_buffer)
                .unwrap();
            tracing::trace!("values written from buffer {:?}", self.write_buffer);
            // TODO test clear write buf
            self.write_buffer.clear();
            Ok(len)
        } else {
            Ok(len)
        }
    }

    pub fn inner(&mut self) -> &mut RedisConnection<S> {
        &mut self.connection
    }
    pub fn into_inner(self) -> RedisConnection<S> {
        self.connection
    }
}
