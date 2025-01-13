use anyhow::anyhow;
use stream::Stream;

use super::*;

pub struct DummyConnection;
impl std::io::Read for DummyConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
impl std::io::Write for DummyConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
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
        todo!()
    }
}

#[derive(Debug)]
pub struct MockConnection {
    input: Vec<crate::connection::Value>,
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
            input: input
                .into_iter()
                .rev()
                .map(|msg| crate::connection::Value::new(msg, 1))
                .collect(),
            expected_output: Some(expected_output.into_iter().rev().collect()),
        }
    }

    pub fn new_input<I>(input: I) -> Self
    where
        I: IntoIterator<Item = resp::Value>,
        <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
    {
        Self {
            input: input
                .into_iter()
                .rev()
                .map(|msg| crate::connection::Value::new(msg, 1))
                .collect(),
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
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
impl std::io::Write for MockConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
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
    fn read_values(&mut self) -> ConnectionResult<Vec<crate::connection::Value>> {
        self.input
            .pop()
            .ok_or(ConnectionError::EndOfInput)
            .map(|v| vec![v])
    }

    fn write_values(&mut self, command: Vec<resp::Value>) -> ConnectionResult<usize> {
        assert_eq!(command.len(), 1);
        let command = command.into_iter().next().unwrap();
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
