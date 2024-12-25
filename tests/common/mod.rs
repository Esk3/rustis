use std::io::Cursor;

use anyhow::bail;
use rustis::{
    connection::client::{ClientService, IntoConnectionToFollowerService},
    io::{Input, Io, NetworkMessage, Output},
    resp::Value,
    service::Service,
};

pub fn test_timeout_millis<F, T>(millis: u64, f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::spawn(f);
    std::thread::sleep(std::time::Duration::from_millis(millis));
    assert!(handle.is_finished(), "test timed out");
    handle.join().unwrap()
}

pub fn test_timeout<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    test_timeout_millis(200, f)
}

pub type MockConnectionToClient = MockInputService<IntoConnectionToFollowerService<ClientService>>;

pub struct MockInputService<S> {
    t: S,
}

impl<S> Service<()> for MockInputService<S> {
    type Response = ();

    fn call(&mut self, request: ()) -> anyhow::Result<Self::Response> {
        todo!()
    }
}

pub struct MockIo {
    input: Vec<Value>,
    output: Vec<Value>,
}

impl MockIo {
    pub fn new(input: impl Into<Vec<Value>>, output: impl Into<Vec<Value>>) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
        }
    }
}

impl Io for MockIo {
    fn read_value(&mut self) -> anyhow::Result<Value> {
        Ok(self.input.pop().unwrap())
    }

    fn write_value(&mut self, value: Value) -> anyhow::Result<usize> {
        todo!()
    }
}
