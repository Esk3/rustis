use crate::connection::{Connection, ConnectionError, ConnectionMessage, ConnectionResult, Input};

use super::{client::ClientHandler, IncomingConnection};

fn setup() -> IncomingConnection<DummyConnection> {
    let connection = DummyConnection;
    IncomingConnection::new(connection)
}

macro_rules! setup {
    () => {
        setup();
    };
    ($input:expr) => {{
        let connection = MockConnection::new_input($input);
        IncomingConnection::new(connection)
    }};
    ($input:expr, $output:expr) => {{
        let connection = MockConnection::new($input, $output);
        IncomingConnection::new(connection)
    }};
}

#[test]
fn create_incoming_connection() {
    let connection = DummyConnection;
    let _ = IncomingConnection::new(connection);
}

#[test]
fn handle_connection_reads_input() {
    let connection = setup!([ConnectionMessage::Input(Input::Ping)].into_iter());
    connection.handle_connection().unwrap();
}

#[test]
fn handler_reads_two_inputs() {
    let connection = setup!([
        ConnectionMessage::Input(Input::Ping),
        ConnectionMessage::Input(Input::Ping)
    ]);
    connection.handle_connection().unwrap();
}
#[test]
#[ignore = "todo"]
fn handler_reads_until_end_of_input() {
    todo!()
}
#[test]
#[ignore = "todo"]
fn connection_calls_client_connection_handler() {
    todo!()
}

#[test]
fn connection_writes_connection_handlers_response() {
    let mut handler = ClientHandler::new();
    let output = [ConnectionMessage::Output(
        handler.handle_request(Input::Ping),
    )];
    let connection = setup!([ConnectionMessage::Input(Input::Ping)], output);
    connection.handle_connection().unwrap();
}

#[test]
#[ignore = "todo"]
fn connection_calls_follower_connection_hanlder_when_connection_is_to_a_follower() {
    todo!()
}

#[test]
#[ignore = "todo"]
fn connection_writes_same_output_as_follower_connection_handler_when_connection_is_to_a_follower() {
    todo!("setup conn, send replconf, send command and match against FollowerHandler")
}

struct DummyConnection;
impl Connection for DummyConnection {
    fn read_resp(&mut self, buf: &mut [u8]) -> ConnectionResult<usize> {
        todo!()
    }

    fn write_resp(&mut self, buf: &[u8]) -> ConnectionResult<()> {
        todo!()
    }

    fn from_connection<C>(value: C) -> Self {
        todo!()
    }

    fn read_command(&mut self) -> ConnectionResult<ConnectionMessage> {
        todo!()
    }

    fn write_command(&mut self, command: ConnectionMessage) -> ConnectionResult<usize> {
        todo!()
    }
}

struct MockConnection {
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
}

impl Connection for MockConnection {
    fn read_resp(&mut self, buf: &mut [u8]) -> ConnectionResult<usize> {
        todo!()
    }

    fn write_resp(&mut self, buf: &[u8]) -> ConnectionResult<()> {
        todo!()
    }

    fn from_connection<C>(value: C) -> Self {
        todo!()
    }

    fn read_command(&mut self) -> ConnectionResult<ConnectionMessage> {
        self.input.pop().ok_or(ConnectionError::EndOfInput)
    }

    fn write_command(&mut self, command: ConnectionMessage) -> ConnectionResult<usize> {
        let Some(ref mut expected) = self.expected_output else {
            return Ok(1);
        };
        let expected = expected.pop().unwrap();
        assert_eq!(command, expected);
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
