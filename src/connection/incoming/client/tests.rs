use super::*;

macro_rules! request_response {
    ($($req:expr),+; $($res:expr),+) => {
    let repo = Repository::new();
    let mut handler = Client::new(repo);
    $(
        let response = handler.handle_request($req).unwrap();
        assert_eq!(response, $res);
    )+
    };
    ($repo:expr ; $($req:expr),+; $($res:expr),+) => {
    let mut handler = Client::new($repo.clone());
    $(
        let response = handler.handle_request($req).unwrap();
        assert_eq!(response, $res);
    )+
    };
}

#[test]
fn create_client_handler() {
    let repo = Repository::new();
    let _: Client = Client::new(repo);
}

#[test]
fn client_handler_handles_request() {
    let repo = Repository::new();
    let mut handler = Client::new(repo);
    let _response = handler.handle_request(Input::Ping).unwrap();
}

#[test]
fn ping_returns_pong() {
    request_response!(Input::Ping ; Output::Pong);
}

#[test]
fn multiple_pings() {
    request_response!(Input::Ping, Input::Ping ; Output::Pong, Output::Pong);
}

#[test]
fn get_returns_get_response() {
    request_response!(Input::Get("abc".into()) ; Output::Get(None));
}

#[test]
fn set_returns_set_response() {
    let (key, value) = ("abc", "xyz");
    request_response!(
        Input::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
            get: false
        };
        Output::Set
    );
}

#[test]
fn set_and_get_stores_values() {
    let repo = Repository::new();
    let (key, value) = ("abc", "xyz");
    request_response!(repo ; Input::Get(key.into()), Input::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
        get: false
    },
    Input::Get(key.into()) ;
    Output::Get(None),
    Output::Set,
    Output::Get(Some(value.into()))
    );
    let some_value = repo.get(key).unwrap();
    assert_eq!(some_value, Some(value.into()));
}

#[test]
fn first_multi_returns_multi_started() {
    request_response!(Input::Multi ; Output::Multi);
}
#[test]
fn multi_returns_already_in_multi() {
    request_response!(Input::Multi, Input::Multi ;
        Output::Multi, Output::MultiError);
}
#[test]
fn client_returns_queue_when_in_multi() {
    let (key, value) = ("abc", "xyz");
    request_response!(Input::Multi,
        Input::Ping,
        Input::Set {
            key: key.into(),
            value: value.into(),
            expiry: None,
            get: false} ;
        Output::Multi,
        Output::Queued,
        Output::Queued);
}
#[test]
fn client_returns_empty_array_when_comitting_empty_multi() {
    request_response!(Input::Multi, Input::CommitMulti ;
        Output::Multi, Output::Array(Vec::new()));
}
#[test]
#[ignore = "todo"]
fn client_returns_array_with_responses_when_comitting_multi() {
    todo!()
}
#[test]
#[ignore = "todo"]
fn repl_conf() {
    todo!()
}
#[test]
#[ignore = "todo"]
fn client_returns_into_follower_on_replconf() {
    todo!()
}

#[test]
#[ignore = "todo"]
fn client_emmits_set_event_on_set_request() {
    // testing:
    //      event manager interface & mock?
    //      event layer with fn that determins the event to send adn test that?
    todo!()
}
