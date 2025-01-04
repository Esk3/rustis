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

    fn read_message(
        &mut self,
    ) -> crate::connection::ConnectionResult<crate::connection::ConnectionMessage> {
        Err(anyhow!("tried to read from dummy connection").into())
    }

    fn write_message(
        &mut self,
        _command: crate::connection::ConnectionMessage,
    ) -> crate::connection::ConnectionResult<usize> {
        Err(anyhow!("tried to write to dummy connection").into())
    }
}

#[derive(Debug)]
pub struct MockConnection {
    input: Vec<ConnectionMessage>,
    expected_output: Option<Vec<ConnectionMessage>>,
}

impl MockConnection {
    pub fn new<I, O>(input: I, expected_output: O) -> Self
    where
        I: IntoIterator<Item = ConnectionMessage>,
        O: IntoIterator<Item = ConnectionMessage>,
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
        I: IntoIterator<Item = ConnectionMessage>,
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
    fn write_message(&mut self, command: ConnectionMessage) -> ConnectionResult<usize> {
        let Some(ref mut expected) = self.expected_output else {
            return Ok(1);
        };
        let expected = expected.pop().unwrap();
        assert_eq!(command, expected);
        Ok(1)
    }

    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self> {
        todo!()
    }

    fn read_message(&mut self) -> ConnectionResult<ConnectionMessage> {
        self.input.pop().ok_or(ConnectionError::EndOfInput)
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
