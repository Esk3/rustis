use super::*;

#[test]
fn create_client_handler() {
    let _: ClientHandler = ClientHandler::new();
}

#[test]
fn client_handler_handles_request() {
    let mut handler = ClientHandler::new();
    let _response = handler.handle_request(());
}
