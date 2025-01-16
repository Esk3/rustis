use std::io::Write;

use crate::resp::{
    self,
    value::{deserialize_value, serialize_value},
};
use stream::Stream;

use super::*;

pub struct DummyConnection;
impl std::io::Read for DummyConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
impl std::io::Write for DummyConnection {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
impl Stream for DummyConnection {
    type Addr = std::net::SocketAddrV4;
    fn connect(_: std::net::SocketAddrV4) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        panic!("tried to connect to dummy connection");
    }

    fn peer_addr(&self) -> Self::Addr {
        std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0)
    }
}

#[derive(Debug)]
pub struct MockConnection {
    input: Vec<resp::Value>,
    expected_output: Option<Vec<resp::Value>>,
}

impl MockConnection {
    pub fn new<I, O>(input: I, expected_output: O) -> Self
    where
        I: IntoIterator<Item = resp::Value>,
        O: IntoIterator<Item = resp::Value>,
        <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
        <O as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
    {
        Self {
            input: input.into_iter().rev().collect(),
            expected_output: Some(expected_output.into_iter().rev().collect()),
        }
    }

    pub fn new_input<I>(input: I) -> Self
    where
        I: IntoIterator<Item = resp::Value>,
        <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
    {
        Self {
            input: input.into_iter().rev().collect(),
            expected_output: None,
        }
    }

    pub(crate) fn empty() -> MockConnection {
        Self {
            input: Vec::new(),
            expected_output: None,
        }
    }
}

impl std::io::Read for MockConnection {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        let value = self.read_value().unwrap();
        let bytes = serialize_value(&value);
        let len = bytes.len();
        buf.write_all(&bytes).unwrap();
        Ok(len)
    }
}

impl std::io::Write for MockConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let value = deserialize_value(buf).unwrap().0;
        self.write_value(value).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
impl Stream for MockConnection {
    type Addr = ();
    fn connect((): ()) -> anyhow::Result<Self> {
        todo!()
    }

    fn peer_addr(&self) -> Self::Addr {
        todo!()
    }
}

impl MockConnection {
    fn read_value(&mut self) -> ConnectionResult<resp::Value> {
        self.input.pop().ok_or(ConnectionError::EndOfInput)
    }

    fn write_value(&mut self, command: resp::Value) -> ConnectionResult<usize> {
        let Some(ref mut expected) = self.expected_output else {
            return Ok(1);
        };
        let expected = expected.pop().unwrap_or_else(|| {
            panic!("mock write failed: no more writes were expected {command:?}")
        });
        assert_eq!(command, expected, "mock write does not match expected");
        Ok(1)
    }
}

impl Drop for MockConnection {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        assert!(self.input.is_empty(), "unused input");
        assert!(
            self.expected_output
                .as_ref()
                .unwrap_or(&Vec::new())
                .is_empty(),
            "expected more output"
        );
    }
}
