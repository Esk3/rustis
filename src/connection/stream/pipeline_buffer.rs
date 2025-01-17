use std::fmt::Debug;

use crate::{
    resp::{self, value::serialize_value},
    Message,
};

use super::{RedisConnection, Stream};

#[derive(Debug)]
pub struct PipelineBuffer<S> {
    pub(super) connection: RedisConnection<S>,
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

    pub fn read(&mut self) -> super::Result<Message<resp::Value>> {
        if let Some(value) = self.read_buffer.pop() {
            tracing::trace!("value read from buffer: [{value:?}]");
            Ok(value)
        } else {
            self.read_buffer
                .extend(self.connection.read_all()?.into_iter().rev());
            tracing::trace!("values read into buffer: {:?}", self.read_buffer);
            tracing::trace!("value read from buffer: [{:?}]", self.read_buffer.last());
            Ok(self.read_buffer.pop().expect("`self.read_buffer` just got filled by reading inner so it should have item(s) to pop"))
        }
    }

    pub fn write(&mut self, value: &resp::Value) -> super::Result<usize> {
        let value = serialize_value(value);
        let len = value.len();
        tracing::trace!(
            "value written to buffer: {value:?}, {:?}",
            String::from_utf8_lossy(&value)
        );
        self.write_buffer.extend(value);
        if self.read_buffer.is_empty() {
            self.connection.inner().write_all(&self.write_buffer)?;
            tracing::trace!(
                "values written from buffer {:?}, {:?}",
                self.write_buffer,
                String::from_utf8_lossy(&self.write_buffer)
            );
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
