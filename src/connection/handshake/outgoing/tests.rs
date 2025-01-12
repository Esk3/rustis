use crate::connection::Connection;

use super::*;

pub fn expected_order() -> [Option<Vec<resp::Value>>; 5] {
    [
        None,
        Some(resp::Value::simple_string("Pong")),
        Some(resp::Value::ok()),
        Some(resp::Value::ok()),
        Some(resp::Value::simple_string("PSYNC")),
    ]
    .map(|v| v.map(|v| vec![v]))
}

#[test]
fn new_handshake_is_not_finished() {
    let handshake = OutgoingHandshake::new();
    assert!(!handshake.is_finished());
}
#[test]
fn incorrect_try_advance_errors() {
    let mut handshake = OutgoingHandshake::new();
    let response = resp::Value::simple_string("SomethingInvalid");
    let err = handshake.try_advance(&Some(vec![response]));
    assert!(err.is_err());
}
#[test]
fn handshake_is_finished_after_five_sucessfull_advances() {
    let mut handshake = OutgoingHandshake::new();
    for message in expected_order() {
        assert!(!handshake.is_finished());
        handshake.try_advance(&message).unwrap();
    }
    assert!(handshake.is_finished());
}
#[test]
fn handshake_try_advance_returns_correct_messages_on_sucessful_advance() {
    let mut handshake = OutgoingHandshake::new();
    let expected_advance_return_value_order = [
        vec![resp::Value::simple_string("PING")],
        resp::Value::bulk_strings("REPLCONF; listing-port; 1"),
        resp::Value::bulk_strings("REPLCONF; CAPA; SYNC"),
        vec![resp::Value::simple_string("PSYNC")],
    ];
    let in_out = expected_order()
        .into_iter()
        .zip(expected_advance_return_value_order);
    for (i, (msg, expected)) in in_out.enumerate() {
        let actual = handshake.try_advance(&msg).unwrap();
        if i > 0 {
            assert_eq!(actual, Some(expected.into_array()), "i: {i}, msg: {msg:?}");
        }
    }
}
#[test]
fn handshake_resets_on_trying_to_advance_on_wrong_message() {
    let mut handshake = OutgoingHandshake::new();
    let messages = expected_order()
        .into_iter()
        .take(4)
        .chain(std::iter::once(Some(vec![resp::Value::simple_string(
            "abc",
        )])))
        .chain(expected_order());
    for (i, message) in messages.enumerate() {
        assert!(!handshake.is_finished(), "i: {i}, msg: {message:?}");
        let res = handshake.try_advance(&message);
        if i != 4 {
            res.unwrap();
        }
    }
    assert!(handshake.is_finished());
}
#[test]
fn handshake_returns_err_on_advancing_after_finish() {
    let mut handshake = OutgoingHandshake::new();
    for msg in expected_order() {
        handshake.try_advance(&msg).unwrap();
    }
    assert!(handshake.is_finished());
    assert!(handshake.try_advance(&None).is_err());
}
#[test]
fn expected_usage() {
    let mut handshake = OutgoingHandshake::new();
    let mut dummy_conn = crate::connection::DummyConnection;
    let mut dummy_responses = expected_order().into_iter().skip(1).map(|msg| msg.unwrap());
    let mut response = None;
    while let Some(next) = handshake.try_advance(&response).unwrap() {
        let dummy_err = dummy_conn.write_values(vec![next]).unwrap_err();
        assert_eq!(dummy_err.to_string(), "tried to write to dummy connection");
        let dummy_err = dummy_conn.read_values().unwrap_err();
        assert_eq!(dummy_err.to_string(), "tried to read from dummy connection");
        response = dummy_responses.next();
    }
    assert!(handshake.is_finished());
}
