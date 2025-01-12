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

    fn read_values(
        &mut self,
    ) -> crate::connection::ConnectionResult<Vec<crate::connection::Value>> {
        Err(anyhow!("tried to read from dummy connection").into())
    }

    fn write_values(
        &mut self,
        _command: Vec<resp::Value>,
    ) -> crate::connection::ConnectionResult<usize> {
        Err(anyhow!("tried to write to dummy connection").into())
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
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

impl Connection for MockConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self> {
        todo!()
    }

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
