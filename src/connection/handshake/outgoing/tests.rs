use super::*;

struct OutgoingHandshakeTest {
    handshake: OutgoingHandshake,
}

impl OutgoingHandshakeTest {
    fn setup() -> Self {
        Self {
            handshake: OutgoingHandshake::new(),
        }
    }
}

macro_rules! handshake_test {
    ($name:ident, $handshake:ident, $body:tt) => {
        #[test]
        fn $name() {
            #[allow(unused_mut)]
            let OutgoingHandshakeTest {
                handshake:
                    mut $handshake,
            } = OutgoingHandshakeTest::setup();
            $body
        }
    };
    ([ok] $name:ident, $handshake:ident, $body:tt) => {
        handshake_test!($name, $handshake, {
            let result = $body;
            assert!(result.is_ok());
        });
    };
    ([err] $name:ident, $handshake:ident, $body:tt) => {
        handshake_test!($name, $handshake, {
            let result = $body;
            assert!(result.is_err());
        });
    };
    ( [$handshake:ident], $( $( [$mod:ident] )? $name:ident, $body:tt );* ) => {
        $(
            handshake_test!( $( [$mod] )? $name, $handshake, $body);
        )*
    };
}

handshake_test! {
    [handshake],
    new_handshake_is_not_finished,  {
        assert!(!handshake.is_finished());
    };
    [err] incorrect_try_advance_errors, {
        let response = Output::Pong;
        handshake.try_advance(response)
    };
    [ok] correct_try_advance_is_ok, {
        let response = Output::Pong;
        handshake.try_advance(response)
    }
}

#[test]
fn get_message() {
    let _: Input = OutgoingHandshake::new().get_message();
}

#[test]
fn handle_response() {
    OutgoingHandshake::new().handle_response(Output::Pong.into());
}

#[test]
fn handshake_messsage_order() {
    let mut handshake = OutgoingHandshake::new();
    let ping = handshake.get_message();
    assert_eq!(ping, Input::Ping);
    handshake.next();
    let replconf = handshake.get_message();
    assert!(matches!(
        replconf,
        Input::ReplConf(ReplConf::ListingPort(1))
    ));

    handshake.next();
    let replconf = handshake.get_message();
    let s = String::new();
    assert!(matches!(replconf, Input::ReplConf(ReplConf::Capa(s))));

    handshake.next();
    let psync = handshake.get_message();
    assert_eq!(psync, Input::Psync);
}

#[test]
fn handshake_steps_on_expected_response() {
    let mut handshake = OutgoingHandshake::new();
    let first = handshake.get_message();
    handshake.handle_response(Output::Pong.into()).unwrap();
    let second = handshake.get_message();
    assert_ne!(first, second);
}

#[test]
fn handshake_is_finished_on_end() {
    let mut handshake = OutgoingHandshake::new();
    handshake.next();
    assert!(!handshake.is_finished());
    handshake.next();
    assert!(!handshake.is_finished());
    handshake.next();
    assert!(!handshake.is_finished());
    handshake.next();
    assert!(handshake.is_finished());
}
