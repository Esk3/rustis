use crate::{repository::Repository, resp};

use super::*;

fn setup() -> Handler {
    let repo = Repository::default();
    Handler::new(repo)
}

fn test_handler<I, O>(input: I, output: O) -> Repository
where
    I: IntoIterator<Item = Input>,
    O: IntoIterator<Item = Option<Output>>,
{
    let repo = Repository::default();
    let mut handler = Handler::new(repo.clone());
    for (req, expected) in input.into_iter().zip(output) {
        let res = handler.handle_request(Request::new(req, 1)).unwrap();
        assert_eq!(res, expected)
    }
    repo
}

#[test]
fn create_handler() {
    let repo = Repository::default();
    let _: Handler = Handler::new(repo);
}

#[test]
fn handle_request() {
    let mut handler = setup();
    _ = handler.handle_request(Request::new(Input::Ping, 0));
}

#[test]
fn handler_returns_none_on_ping() {
    let input = [Input::Ping];
    let output = [None];
    test_handler(input, output);
}

#[test]
fn handler_returns_none_on_set() {
    let set = Input::Set {
        key: String::new(),
        value: String::new(),
        expiry: None,
        get: false,
    };
    let input = [set];
    let output = [None];
    test_handler(input, output);
}

#[test]
fn get_bytes_processed() {
    let handler = setup();
    let bytes_processed: usize = handler.get_bytes_processed();
}

#[test]
fn new_handler_has_processed_zero_bytes() {
    let handler = setup();
    let zero = handler.get_bytes_processed();
    assert_eq!(zero, 0);
}

#[test]
fn add_to_bytes_processed() {
    let mut handler = setup();
    handler.add_processed_bytes(0);
}

#[test]
fn get_bytes_processed_matches_bytes_added() {
    let mut handler = setup();
    let zero = handler.get_bytes_processed();
    assert_eq!(zero, 0);
    handler.add_processed_bytes(1);
    let one = handler.get_bytes_processed();
    assert_eq!(one, 1);

    handler.add_processed_bytes(3);
    let four = handler.get_bytes_processed();
    assert_eq!(four, 4);
}

#[test]
fn handle_updates_bytes_processed_when_handling_request() {
    let mut handler = setup();
    handler
        .handle_request(Request::new(Input::Ping, 1))
        .unwrap();
    let bytes_handled = handler.get_bytes_processed();
    assert_ne!(bytes_handled, 0);
}

#[test]
#[ignore = "todo"]
fn handler_updates_bytes_processed_at_end_of_handle_request() {
    todo!()
}

#[test]
fn handler_repl_conf_get_ack_is_zero_when_no_inputs_made() {
    let input = [ReplConf::GetAck(0).into()];
    let output = [Some(ReplConf::GetAck(0).into())];
    test_handler(input, output);
}

#[test]
fn handler_repl_conf_get_ack_is_more_than_zero_with_inputs_made() {
    let input = [Input::Ping, ReplConf::GetAck(0).into()];
    let output = [None, Some(ReplConf::GetAck(1).into())];
    test_handler(input, output);
}

#[test]
fn handler_writes_to_repo_on_set() {
    let repo = Repository::default();
    let (key, value) = ("abc", "xyz");
    let mut handler = Handler::new(repo.clone());
    handler
        .handle_request(Request::new(
            Input::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false,
            },
            0,
        ))
        .unwrap();
    let some_value = repo.get(key, std::time::SystemTime::UNIX_EPOCH).unwrap();
    assert_eq!(some_value, Some(value.into()));
}

#[test]
#[ignore = "todo"]
fn handler_emitts_event_on_set() {
    todo!()
}
