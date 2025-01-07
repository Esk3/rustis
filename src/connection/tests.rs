use anyhow::anyhow;

use super::*;

pub struct DummyConnection;
impl Connection for DummyConnection {
    fn connect(addr: std::net::SocketAddr) -> crate::connection::ConnectionResult<Self>
    where
        Self: Sized,
    {
        Err(anyhow!("tried to connect to dummy connection").into())
    }

    fn read_message(&mut self) -> crate::connection::ConnectionResult<crate::resp::Message> {
        Err(anyhow!("tried to read from dummy connection").into())
    }

    fn write_message(
        &mut self,
        _command: resp::Message,
    ) -> crate::connection::ConnectionResult<usize> {
        Err(anyhow!("tried to write to dummy connection").into())
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        todo!()
    }
}

#[derive(Debug)]
pub struct MockConnection {
    input: Vec<resp::Message>,
    expected_output: Option<Vec<resp::Message>>,
}

impl MockConnection {
    pub fn new<I, O>(input: I, expected_output: O) -> Self
    where
        I: IntoIterator<Item = resp::Message>,
        O: IntoIterator<Item = resp::Message>,
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
        I: IntoIterator<Item = resp::Message>,
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

impl Connection for MockConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self> {
        todo!()
    }

    fn read_message(&mut self) -> ConnectionResult<resp::Message> {
        self.input.pop().ok_or(ConnectionError::EndOfInput)
    }

    fn write_message(&mut self, command: resp::Message) -> ConnectionResult<usize> {
        let Some(ref mut expected) = self.expected_output else {
            return Ok(1);
        };
        let expected = expected.pop().unwrap_or_else(|| {
            panic!("mock write failed: no more writes were expected {command:?}")
        });
        assert_eq!(command, expected, "mock write does not match expected");
        Ok(1)
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        todo!()
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
