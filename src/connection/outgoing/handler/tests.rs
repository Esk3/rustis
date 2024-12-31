use crate::connection::Input;

use super::*;

fn setup() -> Handler {
    Handler::new()
}

#[test]
fn create_handler() {
    let _: Handler = Handler::new();
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
