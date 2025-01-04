use crate::{connection::Connection, test_helper};

use super::*;

pub const EXPECTED_ORDER: [Option<Output>; 5] = [
    None,
    Some(Output::Pong),
    Some(Output::ReplConf(ReplConf::Ok)),
    Some(Output::ReplConf(ReplConf::Ok)),
    Some(Output::Psync),
];

test_helper! {
    OutgoingHandshakeTest, { handshake: OutgoingHandshake, OutgoingHandshake::new() },
    [false]
    new_handshake_is_not_finished,  {
        handshake.is_finished()
    };
    [err]
    incorrect_try_advance_errors, {
        let response = Output::Get(None);
        handshake.try_advance(&Some(response))
    };
    [true]
    handshake_is_finished_after_five_sucessfull_advances, {
        for message in EXPECTED_ORDER {
            assert!(!handshake.is_finished());
            handshake.try_advance(&message).unwrap();
        }
        handshake.is_finished()
    };
    handshake_try_advance_returns_correct_messages_on_sucessful_advance, {
        let expected_advance_return_value_order = [
            Input::Ping,
            ReplConf::ListingPort(1).into(),
            ReplConf::Capa(String::new()).into(),
            Input::Psync,
        ];
        let in_out = EXPECTED_ORDER.into_iter().zip(expected_advance_return_value_order.into_iter());
        for (i, (msg, expected)) in in_out.enumerate() {
            let actual = handshake.try_advance(&msg).unwrap();
            assert_eq!(actual, expected, "i: {i}, msg: {msg:?}");
        }
    };
    [true]
    handshake_resets_on_trying_to_advance_on_wrong_message, {
        let messages = EXPECTED_ORDER.into_iter()
            .take(4)
            .chain(std::iter::once(Some(Output::Get(None))))
            .chain(EXPECTED_ORDER);
        for (i, message) in messages.enumerate() {
            assert!(!handshake.is_finished(), "i: {i}, msg: {message:?}");
            let res = handshake.try_advance(&message);
            if i != 4 { res.unwrap(); }
        }
        handshake.is_finished()
    };
    [err]
    handshake_returns_err_on_advancing_after_finish, {
        for msg in EXPECTED_ORDER {
            handshake.try_advance(&msg).unwrap();
        }
        assert!(handshake.is_finished());
        handshake.try_advance(&None)
    };
    expected_usage, {
        let mut dummy_conn = crate::connection::DummyConnection;
        let mut dummy_responses = EXPECTED_ORDER.into_iter().skip(1).map(|msg| msg.unwrap());
        let mut response = None;
        while !handshake.is_finished() {
            let message = handshake.try_advance(&response).unwrap();
            let dummy_err = dummy_conn.write_message(message.into()).unwrap_err();
            assert_eq!(dummy_err.to_string(), "tried to write to dummy connection");
            let dummy_err = dummy_conn.read_message().unwrap_err();
            assert_eq!(dummy_err.to_string(), "tried to read from dummy connection");
            response = dummy_responses.next();
        }
    };
}
