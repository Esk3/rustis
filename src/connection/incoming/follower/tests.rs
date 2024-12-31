use crate::event;

use super::*;

fn setup() -> Follower {
    Follower::new()
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

// give follower connection/mut connection on handle_event?
