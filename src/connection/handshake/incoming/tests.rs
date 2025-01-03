use super::*;

#[test]
fn create_incoming_handshake() {
    let _handshake: IncomingHandshake = IncomingHandshake::new();
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
