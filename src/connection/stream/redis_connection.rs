use std::fmt::Debug;

use crate::{
    resp::{
        self,
        value::{deserialize_value, serialize_value},
    },
    Message,
};

use super::Stream;

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
            tracing::trace!("reading from stream");
            let bytes_read = self.stream.read(&mut self.buf).unwrap();
            tracing::trace!(
                "read from stream: {:?}",
                String::from_utf8_lossy(&self.buf[..bytes_read])
            );
            self.i = bytes_read;
        }
        let (value, bytes_consumed) = deserialize_value(&self.buf).unwrap();
        tracing::trace!("deserialized value: {value:?}");
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
        tracing::trace!("serializing value: {value:?}");
        let bytes = serialize_value(value);
        tracing::trace!(
            "value serialized: {bytes:?}, {:?}",
            String::from_utf8_lossy(&bytes)
        );
        self.stream.write_all(&bytes).unwrap();
        self.stream.flush().unwrap();
        tracing::trace!(
            "serialized value flushed to stream. length: {}",
            bytes.len()
        );
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
