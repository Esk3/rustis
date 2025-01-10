use super::*;

#[must_use]
pub fn expected_input() -> [Vec<resp::Value>; 4] {
    [
        "PING",
        "REPLCONF; listing-port; 1",
        "REPLCONF; capa fullresync",
        "PSYNC",
    ]
    .map(resp::Value::bulk_strings)
}

#[must_use]
pub fn expected_output() -> [Vec<resp::Value>; 4] {
    resp::Value::bulk_strings("PONG; OK; OK; PSYNC")
        .into_iter()
        .map(|v| vec![v])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[test]
fn new_handshake_is_not_finished() {
    let handshake = IncomingHandshake::new();
    assert!(!handshake.is_finished());
}

#[test]
fn can_start_with_ping() {
    let mut handshake = IncomingHandshake::new();
    let first = expected_input()[0].clone();
    let ok = handshake.try_advance(&first);
    assert!(ok.is_ok());
}
#[test]
fn can_start_with_repl_conf_listing_port() {
    let mut handshake = IncomingHandshake::new();
    let other_valid_first = expected_input()[1].clone();
    let ok = handshake.try_advance(&other_valid_first);
    assert!(ok.is_ok());
}
#[test]
fn invalid_start_input_is_err() {
    let mut handshake = IncomingHandshake::new();
    let err = handshake.try_advance(&[resp::Value::bulk_string("MULTI")]);
    assert!(err.is_err());
}

#[test]
fn expected_use() {
    let mut handshake = IncomingHandshake::new();
    let dummy_input = expected_input();
    let expected_output = expected_output();
    for (i, (input, expected_response)) in dummy_input.into_iter().zip(expected_output).enumerate()
    {
        assert!(!handshake.is_finished());
        dbg!(&input, &expected_response);
        let response = handshake.try_advance(&input).unwrap();
        assert!(
            response.eq_ignore_ascii_case(&expected_response[0].clone().expect_string().unwrap()),
            "{i}, left: {input:?} != right: {expected_response:?}"
        );
    }
    assert!(handshake.is_finished());
}
