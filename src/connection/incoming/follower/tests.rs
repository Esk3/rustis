use crate::{connection::MockConnection, event, test_helper};

use super::*;

test_helper! {
    FollowerTest { follower: Follower, Follower::new()}
    handle_event() {
        let (key, value) = ("abc", "xyz");
        let kind = event::Kind::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
        };
        let _ = follower.handle_event(kind);
    };
    follower_returns_message() {
        let (key, value) = ("abc", "xyz");
        let kind = event::Kind::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
        };
        let _: anyhow::Result<Option<resp::Value>> = follower.handle_event(kind);
    };
    set_event_returns_set_message() {
        //let (key, value) = ("abc", "xyz");
        //let kind = event::Kind::Set {
        //    key: key.into(),
        //    value: value.into(),
        //    expiry: None,
        //};
        //let response = follower.handle_event(kind).unwrap().unwrap();
        //assert_eq!(
        //    response,
        //    Message::Input(crate::resp::Input::Set {
        //        key: key.into(),
        //        value: value.into(),
        //        expiry: None,
        //        get: false
        //    })
        //);
    };
    follower_uses_incoming_handshake() {
        let input = crate::connection::handshake::incoming::tests::EXPECTED_INPUT;
        let output = crate::connection::handshake::incoming::tests::EXPECTED_OUTPUT;
        let mut connection = MockConnection::new(
            input
            .into_iter()
            .map(std::convert::Into::into)
            .collect::<Vec<_>>(),
            output
            .into_iter()
            .map(std::convert::Into::into)
            .collect::<Vec<_>>(),
        );
        follower.handshake(&mut connection).unwrap();
    };
}

fn setup() -> Follower {
    Follower::new()
}

macro_rules! setup {
    ($follower:ident) => {
        let mut $follower = Follower::new();
    };
}

#[test]
#[should_panic(expected = "end of input")]
fn follower_recives_handshake() {
    let mut follower = setup();
    follower.handshake(&mut MockConnection::empty()).unwrap();
}

#[test]
#[should_panic]
fn follower_panics_on_invalid_handshake() {
    //setup!(follower);
    //let mut connection = MockConnection::new(
    //    [Input::Ping.into(), Input::Ping.into()],
    //    [Output::Pong.into()],
    //);
    //follower.handshake(&mut connection).unwrap();
}

#[test]
#[ignore = "todo"]
fn returns_bytes_processed_after_no_messages() {
    todo!()
}

#[test]
#[ignore = "todo"]
fn returns_bytes_processed_after_messages() {
    todo!()
}
