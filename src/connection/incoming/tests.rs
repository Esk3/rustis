use crate::{
    connection::ConnectionResult,
    event,
    resp::{self, Input, Output, ReplConf},
};

use super::super::MockConnection;
use super::*;

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
    ($connection:ident, emitter: $emitter:ident $(, $input:expr, $output:expr)?) => {
        let connection = DummyConnection;
        $(
            let connection = MockConnection::new(
                $input.into_iter().map(std::convert::Into::into).collect::<Vec<resp::Message>>(),
            $output.into_iter().map(std::convert::Into::into).collect::<Vec<resp::Message>>()
            );
        )?
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
    let connection = setup!([Message::Input(Input::Ping)].into_iter());
    connection.run_handler().unwrap();
}

#[test]
fn handler_reads_two_inputs() {
    let connection = setup!([Message::Input(Input::Ping), Message::Input(Input::Ping)]);
    connection.run_handler().unwrap();
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
    let output = [Message::Output(
        handler
            .handle_request(client::Request::epoch(Input::Ping, 0))
            .unwrap()
            .into_output()
            .unwrap(),
    )];
    let connection = setup!([Message::Input(Input::Ping)], output);
    connection.run_handler().unwrap();
}

#[test]
fn handle_client_connection_returns_ok_on_replconf() {
    let mut connection = setup!(
        [Input::ReplConf(crate::resp::ReplConf::ListingPort(1)).into()],
        []
    );
    connection.handle_client_connection().unwrap();
}

#[test]
fn connection_calls_follower_connection_hanlder_when_connection_is_to_a_follower() {
    let mut follower = Follower::new();
    let event = event::Kind::Set {
        key: "abc".into(),
        value: "qwerty".into(),
        expiry: None,
    };
    let response = follower.handle_event(event.clone()).unwrap().unwrap();
    setup!(
        connection,
        emitter: emitter,
        [Input::ReplConf(ReplConf::ListingPort(1))],
        [response]
    );
    let handle = std::thread::spawn(move || {
        connection.run_handler().unwrap();
    });
    std::thread::sleep(std::time::Duration::from_millis(1));
    emitter.emmit(event.clone());
    std::thread::sleep(std::time::Duration::from_millis(1));
    assert!(handle.is_finished());
    handle.join().unwrap();
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
    setup!(_connection, emitter: emitter);
    let event = event::Kind::Set {
        key: "abc".into(),
        value: "efg".into(),
        expiry: None,
    };
    todo!()
}

struct DummyConnection;
impl Connection for DummyConnection {
    fn connect(addr: std::net::SocketAddr) -> ConnectionResult<Self> {
        todo!()
    }

    fn read_message(&mut self) -> ConnectionResult<Message> {
        todo!()
    }

    fn write_message(&mut self, command: Message) -> ConnectionResult<usize> {
        todo!()
    }

    fn get_peer_addr(&self) -> std::net::SocketAddr {
        todo!()
    }
}
