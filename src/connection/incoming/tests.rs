use crate::{
    connection::{Connection, ConnectionError, ConnectionMessage, ConnectionResult, Input},
    event::{self, EventEmitter},
    repository::Repository,
};

use super::{
    client::{Client, ClientRequest},
    IncomingConnection,
};

fn dummy_setup() -> IncomingConnection<DummyConnection> {
    let connection = DummyConnection;
    let repo = Repository::new();
    let emitter = EventEmitter::new();
    IncomingConnection::new(connection, emitter, repo)
}

macro_rules! setup {
    () => {
        setup();
    };
    ($connection:ident) => {
        let $connection = dummy_setup();
    };
    ($connection:ident, emitter: $emitter:ident) => {
        let connection = DummyConnection;
        let repo = Repository::new();
        let $emitter = EventEmitter::new();
        let $connection = IncomingConnection::new(connection, $emitter.clone(), repo);
    };
    ($input:expr) => {{
        let connection = MockConnection::new_input($input);
        let repo = Repository::new();
        let emitter = EventEmitter::new();
        IncomingConnection::new(connection, emitter, repo)
    }};
    ($input:expr, $output:expr) => {{
        let connection = MockConnection::new($input, $output);
        let repo = Repository::new();
        let emitter = EventEmitter::new();
        IncomingConnection::new(connection, emitter, repo)
    }};
}

#[test]
fn create_incoming_connection() {
    let connection = DummyConnection;
    let repo = Repository::new();
    let emitter = EventEmitter::new();
    let _ = IncomingConnection::new(connection, emitter, repo);
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
    let repo = Repository::new();
    let emitter = EventEmitter::new();
    let mut handler = Client::new(emitter, repo);
    let output = [ConnectionMessage::Output(
        handler
            .handle_request(ClientRequest::now(Input::Ping, 0))
            .unwrap(),
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

#[test]
fn handle_follower_connection_writes_same_output_as_follower_handler() {
    setup!(connection, emitter: emitter);
    let event = event::Kind::Set {
        key: "abc".into(),
        value: "efg".into(),
        expiry: None,
    };
}

struct DummyConnection;
impl Connection for DummyConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self> {
        todo!()
    }

    fn read_message(&mut self) -> ConnectionResult<ConnectionMessage> {
        todo!()
    }

    fn write_message(&mut self, command: ConnectionMessage) -> ConnectionResult<usize> {
        todo!()
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
