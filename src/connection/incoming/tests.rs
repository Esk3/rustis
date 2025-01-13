use client_connection::client::default_router;

use crate::{
    connection::{self, ConnectionResult, DummyConnection},
    event, repository, resp,
};

use super::super::MockConnection;
use super::*;

fn dummy_setup() -> IncomingConnection<DummyConnection> {
    let connection = DummyConnection;
    let repo = Repository::default();
    let emitter = EventEmitter::new();
    IncomingConnection::new(connection, default_router(), emitter, repo)
}

struct Tester {
    connection: IncomingConnection<MockConnection>,
    emitter: EventEmitter,
    repo: Repository,
}

impl Tester {
    fn setup<I, O>(input: I, expected_output: O) -> Self
    where
        I: IntoIterator<Item = resp::Value>,
        O: IntoIterator<Item = resp::Value>,
        <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
        <O as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
    {
        let repo = Repository::default();
        let emitter = EventEmitter::new();
        Self {
            connection: IncomingConnection::new(
                MockConnection::new(input, expected_output),
                default_router(),
                emitter.clone(),
                repo.clone(),
            ),
            emitter,
            repo,
        }
    }
    fn run(self) -> anyhow::Result<()> {
        self.connection.run_handler()
    }
}

#[test]
fn create_incoming_connection() {
    let connection = DummyConnection;
    let repo = Repository::default();
    let emitter = EventEmitter::new();
    let _ = IncomingConnection::new(connection, default_router(), emitter, repo);
}

#[test]
#[should_panic(expected = "EndOfInput")]
fn handle_connection_reads_input() {
    let Tester { connection, .. } = Tester::setup(
        [resp::Value::simple_string("PING")],
        [resp::Value::simple_string("PONG")],
    );
    connection.run_handler().unwrap();
}

#[test]
#[should_panic(expected = "EndOfInput")]
fn handler_reads_two_inputs() {
    let tester = Tester::setup(
        [
            resp::Value::simple_string("PING"),
            resp::Value::simple_string("PING"),
        ],
        [
            resp::Value::simple_string("PONG"),
            resp::Value::simple_string("PONG"),
        ],
    );
    tester.run().unwrap();
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
    //let repo = Repository::default();
    //let emitter = EventEmitter::new();
    //let mut handler = Client::new(emitter, repo);
    //let output = [Message::Output(
    //    handler
    //        .handle_request(client::Request::epoch(Input::Ping, 0))
    //        .unwrap()
    //        .into_output()
    //        .unwrap(),
    //)];
    //let connection = setup!([Message::Input(Input::Ping)], output);
    //connection.run_handler().unwrap();
}

#[test]
fn handle_client_connection_returns_ok_on_replconf() {
    //let mut connection = setup!(
    //    [Input::ReplConf(crate::resp::ReplConf::ListingPort(1)).into()],
    //    []
    //);
    //connection.handle_client_connection().unwrap();
}

#[test]
fn connection_calls_follower_connection_hanlder_when_connection_is_to_a_follower() {
    //let mut follower = Follower::new();
    //let event = event::Kind::Set {
    //    key: "abc".into(),
    //    value: "qwerty".into(),
    //    expiry: None,
    //};
    //let response = follower.handle_event(event.clone()).unwrap().unwrap();
    //setup!(
    //    connection,
    //    emitter: emitter,
    //    [Input::ReplConf(ReplConf::ListingPort(1))],
    //    [response]
    //);
    //let handle = std::thread::spawn(move || {
    //    connection.run_handler().unwrap();
    //});
    //std::thread::sleep(std::time::Duration::from_millis(1));
    //emitter.emmit(event.clone());
    //std::thread::sleep(std::time::Duration::from_millis(1));
    //assert!(handle.is_finished());
    //handle.join().unwrap();
}

#[test]
#[ignore = "todo"]
fn connection_preformes_follower_handshake_on_replconf() {
    todo!()
}

#[test]
#[ignore = "todo"]
fn connection_writes_same_output_as_follower_connection_handler_when_connection_is_to_a_follower() {
    todo!("setup conn, send replconf, send command and match against FollowerHandler")
}

#[test]
#[ignore = "todo"]
fn handle_follower_connection_writes_same_output_as_follower_handler() {
    //setup!(_connection, emitter: emitter);
    //let event = event::Kind::Set {
    //    key: "abc".into(),
    //    value: "efg".into(),
    //    expiry: None,
    //};
    //todo!()
}
