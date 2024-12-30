use crate::{Connection, ConnectionError, ConnectionResult};

use super::{client::ClientHandler, IncomingConnection};

fn setup() -> IncomingConnection<DummyConnection> {
    let connection = DummyConnection;
    IncomingConnection::new(connection)
}

fn mock_setup(
    input: impl Into<Vec<()>>,
    output: Option<impl Into<Vec<()>>>,
) -> IncomingConnection<MockConnection> {
    let connection = MockConnection::new(input, output);
    IncomingConnection::new(connection)
}

#[test]
fn create_incoming_connection() {
    let connection = DummyConnection;
    let _ = IncomingConnection::new(connection);
}

#[test]
fn handle_connection_reads_input() {
    let connection = mock_setup(vec![()], None::<Vec<()>>);
    connection.handle_connection().unwrap();
}

#[test]
fn handler_reads_two_inputs() {
    let connection = mock_setup(vec![(), ()], None::<Vec<()>>);
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
    let output = [handler.handle_request(())];
    let connection = mock_setup([()], Some(output));
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

    fn read_command(&mut self) -> ConnectionResult<()> {
        todo!()
    }

    fn write_command(&mut self, command: ()) -> ConnectionResult<()> {
        todo!()
    }
}

struct MockConnection {
    input: Vec<()>,
    expected_output: Option<Vec<()>>,
}

impl MockConnection {
    pub fn new(input: impl Into<Vec<()>>, expected_output: Option<impl Into<Vec<()>>>) -> Self {
        Self {
            input: input.into(),
            expected_output: expected_output.map(|o| o.into()),
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

    fn read_command(&mut self) -> ConnectionResult<()> {
        self.input.pop().ok_or(ConnectionError::EndOfInput)
    }

    fn write_command(&mut self, command: ()) -> ConnectionResult<()> {
        let Some(ref mut expected) = self.expected_output else {
            return Ok(());
        };
        let expected = expected.pop().unwrap();
        //assert_eq!(command, expected);
        Ok(())
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
