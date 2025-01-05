use super::*;
use crate::connection::DummyConnection;
use std::{net::SocketAddrV4, str::FromStr};

use crate::{
    connection::{Connection, MockConnection},
    resp::{Message, Output, ReplConf},
};

fn setup<I, O>(input: I, expected_output: O) -> OutgoingConnection<MockConnection>
where
    I: IntoIterator<Item = Message>,
    O: IntoIterator<Item = Message>,
    <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
    <O as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
{
    let connection = MockConnection::new(input, expected_output);
    OutgoingConnection::new(connection)
}

#[test]
#[should_panic(expected = "tried to connect to dummy connection")]
fn create_outgoing_connection() {
    let _connection: OutgoingConnection<DummyConnection> =
        OutgoingConnection::connect(SocketAddrV4::from_str("127.0.0.1:6739").unwrap().into())
            .unwrap();
}

#[test]
fn create_outgoing_handshake() {
    let _handshake = OutgoingHandshake::new();
}

#[test]
fn handshake_send_replconf_to_connection() {
    let handshake = crate::connection::handshake::outgoing::tests::EXPECTED_ORDER
        .into_iter()
        .filter_map(|msg| msg.map(std::convert::Into::into))
        .collect::<Vec<Message>>();
    let mut connection = setup(
        [
            Output::Pong.into(),
            Output::ReplConf(ReplConf::ListingPort(1)).into(),
            Output::ReplConf(ReplConf::Capa(String::new())).into(),
            Output::Psync.into(),
        ],
        handshake,
    );
    connection.handshake().unwrap();
}

#[test]
#[should_panic]
fn run_sends_handshake() {
    let handshake = crate::connection::handshake::outgoing::tests::EXPECTED_ORDER
        .into_iter()
        .map(|msg| msg.unwrap().into())
        .collect::<Vec<Message>>();
    let connection = setup([], handshake);
    connection.run().unwrap();
}

#[test]
fn run_reads_input_after_handshake() {
    let handshake = crate::connection::handshake::outgoing::tests::EXPECTED_ORDER
        .into_iter()
        .filter_map(|msg| msg.map(std::convert::Into::into))
        .collect::<Vec<Message>>();
    let connection = setup(
        [
            Output::Pong.into(),
            Output::ReplConf(ReplConf::ListingPort(1)).into(),
            Output::ReplConf(ReplConf::Capa(String::new())).into(),
            Output::Psync.into(),
            Input::Ping.into(),
        ],
        handshake,
    );
    connection.run().unwrap();
}
