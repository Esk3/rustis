use crate::{connection::incoming::tests::MockConnection, event};

use super::*;

fn setup() -> Follower {
    Follower::new()
}

macro_rules! setup {
    ($follower:ident) => {
        let mut $follower = Follower::new();
    };
}

#[test]
fn create_follower() {
    let _: Follower = Follower::new();
}

#[test]
fn follower_handles_event() {
    let mut follower = setup();
    let (key, value) = ("abc", "xyz");
    let kind = event::Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: (),
    };
    let _ = follower.handle_event(kind);
}

#[test]
fn follower_returns_message() {
    let mut follower = setup();
    let (key, value) = ("abc", "xyz");
    let kind = event::Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: (),
    };
    let _: anyhow::Result<Option<ConnectionMessage>> = follower.handle_event(kind);
}

#[test]
fn set_event_returns_set_message() {
    let mut follower = setup();
    let (key, value) = ("abc", "xyz");
    let kind = event::Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: (),
    };
    let response = follower.handle_event(kind).unwrap().unwrap();
    assert_eq!(
        response,
        ConnectionMessage::Input(crate::connection::Input::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
            get: false
        })
    );
}

#[test]
#[should_panic(expected = "EndOfInput")]
fn follower_recives_handshake() {
    let mut follower = setup();
    follower.handshake(&mut MockConnection::empty()).unwrap();
}

#[test]
fn follower_uses_incoming_handshake() {
    let handshake = IncomingHandshake::new();
    setup!(follower);
    let mut connection = MockConnection::new(
        handshake
            .get_all_messages()
            .into_iter()
            .map(std::convert::Into::into)
            .collect::<Vec<_>>(),
        handshake
            .get_all_responses()
            .into_iter()
            .map(std::convert::Into::into)
            .collect::<Vec<_>>(),
    );
    follower.handshake(&mut connection).unwrap();
}

#[test]
#[should_panic]
fn follower_panics_on_invalid_handshake() {
    setup!(follower);
    let mut connection = MockConnection::new(
        [Input::Ping.into(), Input::Ping.into()],
        [Output::Pong.into()],
    );
    follower.handshake(&mut connection).unwrap();
}

#[test]
fn create_incoming_handshake() {
    let handshake: IncomingHandshake = IncomingHandshake::new();
}

#[test]
fn get_all_handshake_messages() {
    let messages = IncomingHandshake::new().get_all_messages();
    assert_eq!(
        messages,
        [
            Input::Ping,
            ReplConf::ListingPort(1).into(),
            ReplConf::Capa(String::new()).into(),
            Input::Psync,
        ]
    );
}

#[test]
#[ignore = "todo"]
fn handshake_returns_messages_in_correct_order() {
    todo!()
}

#[test]
fn handshake_returns_ok_and_advances_on_correct_response() {
    let mut handshake = IncomingHandshake::new();
    let first: Option<Output> = handshake.get_message();
    handshake.handle_message_recived(Input::Ping).unwrap();
    let second = handshake.get_message();
    assert_ne!(first, second);
}

#[test]
fn handshake_returns_err_and_does_not_advances_on_wrong_response() {
    let mut handshake = IncomingHandshake::new();
    let first = handshake.get_message();
    assert!(handshake
        .handle_message_recived(Input::Get(String::new()))
        .is_err());
    let second = handshake.get_message();
    assert_eq!(first, second);
}

// give follower connection/mut connection on handle_event?
