use super::*;
use std::{net::SocketAddrV4, str::FromStr};

use crate::connection::{
    incoming::tests::MockConnection, Connection, ConnectionMessage, Output, ReplConf,
};

fn setup<I, O>(input: I, expected_output: O) -> OutgoingConnection<MockConnection>
where
    I: IntoIterator<Item = ConnectionMessage>,
    O: IntoIterator<Item = ConnectionMessage>,
    <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
    <O as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
{
    let connection = MockConnection::new(input, expected_output);
    OutgoingConnection::new(connection)
}

#[test]
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
    let handshake: Vec<ConnectionMessage> = OutgoingHandshake::new()
        .get_all_messages()
        .into_iter()
        .map(std::convert::Into::into)
        .collect();
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
    let handshake: Vec<ConnectionMessage> = OutgoingHandshake::new()
        .get_all_messages()
        .into_iter()
        .map(std::convert::Into::into)
        .collect();
    let connection = setup([], handshake);
    connection.run().unwrap();
}

#[test]
fn run_reads_input_after_handshake() {
    let handshake: Vec<ConnectionMessage> = OutgoingHandshake::new()
        .get_all_messages()
        .into_iter()
        .map(std::convert::Into::into)
        .collect::<Vec<ConnectionMessage>>();
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

struct DummyConnection;
impl Connection for DummyConnection {
    fn connect(addr: std::net::SocketAddr) -> crate::connection::ConnectionResult<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn read_message(
        &mut self,
    ) -> crate::connection::ConnectionResult<crate::connection::ConnectionMessage> {
        todo!()
    }

    fn write_message(
        &mut self,
        command: crate::connection::ConnectionMessage,
    ) -> crate::connection::ConnectionResult<usize> {
        todo!()
    }
}
