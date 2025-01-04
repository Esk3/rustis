use super::*;

struct IncomingHandshakeTest {
    handshake: IncomingHandshake,
}

impl IncomingHandshakeTest {
    fn setup() -> Self {
        Self {
            handshake: IncomingHandshake::new(),
        }
    }
}

macro_rules! handshake_test {
    ($name:ident, $handshake:ident, $body:tt) => {
        #[test]
        fn $name() {
            #[allow(unused_mut, unused_variables)]
            let IncomingHandshakeTest{
                handshake:
                    mut $handshake,
            } = IncomingHandshakeTest::setup();
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
    ( {$handshake:ident}, $( $( [$mod:ident] )? $name:ident, $body:tt );* $(;)? ) => {
        $(
            handshake_test!( $( [$mod] )? $name, $handshake, $body);
        )*
    };
}

pub const EXPECTED_INPUT: [Input; 4] = [
    Input::Ping,
    Input::ReplConf(ReplConf::ListingPort(1)),
    Input::ReplConf(ReplConf::Capa(String::new())),
    Input::Psync,
];

pub const EXPECTED_OUTPUT: [Output; 4] = [
    Output::Pong,
    Output::ReplConf(ReplConf::Ok),
    Output::ReplConf(ReplConf::Ok),
    Output::Psync,
];

handshake_test! {
    { handshake },
    new_handshake_is_not_finished,  {
        assert!(!handshake.is_finished());
    };
    [ok]
    can_start_with_ping, {
        handshake.try_advance(&Input::Ping)
    };
    [ok]
    can_start_with_repl_conf_listing_port, {
        handshake.try_advance(&Input::ReplConf(ReplConf::ListingPort(1)))
    };
    [err]
    invalid_start_input_is_err, {
        handshake.try_advance(&Input::Multi)
    };
    expected_use, {
        let dummy_input = [
            Input::Ping,
            Input::ReplConf(ReplConf::ListingPort(1)),
            Input::ReplConf(ReplConf::Capa(String::new())),
            Input::Psync
        ];
        let expected_output = [
            Output::Pong,
            Output::ReplConf(ReplConf::Ok),
            Output::ReplConf(ReplConf::Ok),
            Output::Psync
        ];
        for (i, (input, expected_response)) in dummy_input.into_iter().zip(expected_output).enumerate() {
            assert!(!handshake.is_finished());
            let response = handshake.try_advance(&input).unwrap();
            assert_eq!(response, expected_response,
                "{i}, {input:?}, {expected_response:?}");
        }
        assert!(handshake.is_finished());
    }
}
