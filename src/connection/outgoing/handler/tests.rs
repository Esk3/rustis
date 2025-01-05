use crate::{repository::Repository, resp::Input};

use super::*;

fn setup() -> Handler {
    let repo = Repository::new();
    Handler::new(repo)
}

#[test]
fn create_handler() {
    let repo = Repository::new();
    let _: Handler = Handler::new(repo);
}

#[test]
fn handle_request() {
    let mut handler = setup();
    _ = handler.handle_request(Input::Ping);
}

#[test]
fn handler_returns_none_on_ping() {
    let mut handler = setup();
    let response = handler.handle_request(Input::Ping).unwrap();
    assert_eq!(response, None);
}

#[test]
fn handler_returns_none_on_set() {
    let mut handler = setup();
    let response = handler
        .handle_request(Input::Set {
            key: String::new(),
            value: String::new(),
            expiry: None,
            get: false,
        })
        .unwrap();
    assert_eq!(response, None);
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
    handler.handle_request(Input::Ping).unwrap();
    let bytes_handled = handler.get_bytes_processed();
    assert_ne!(bytes_handled, 0);
}

#[test]
#[ignore = "todo"]
fn handler_updates_bytes_processed_at_end_of_handle_request() {
    todo!()
}

#[test]
fn handler_writes_to_repo_on_set() {
    let repo = Repository::new();
    let (key, value) = ("abc", "xyz");
    let mut handler = Handler::new(repo.clone());
    handler
        .handle_request(Input::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
            get: false,
        })
        .unwrap();
    let some_value = repo.get(key, std::time::SystemTime::now()).unwrap();
    assert_eq!(some_value, Some(value.into()));
}

#[test]
#[ignore = "todo"]
fn handler_emitts_event_on_set() {
    todo!()
}
