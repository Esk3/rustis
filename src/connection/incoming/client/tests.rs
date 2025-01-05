use crate::{connection::incoming::client, event::EventEmitter, resp::ReplConf};

use super::*;

macro_rules! request_response {
    ($($req:expr),+; $($res:expr),+) => {
    let repo = Repository::new();
    let emitter = EventEmitter::new();
    let mut handler = Client::new(emitter, repo);
    $(
        let response = handler.handle_request(client::Request::now($req,0)).unwrap();
        assert_eq!(response, client::Response::SendOutput($res));
    )+
    };
    ($repo:expr ; $($req:expr),+; $($res:expr),+) => {
    let emitter = EventEmitter::new();
    let mut handler = Client::new(emitter, $repo.clone());
    $(
        let response = handler.handle_request(client::Request::now($req, 0)).unwrap();
        assert_eq!(response, client::Response::SendOutput($res));
    )+
    };
}

#[test]
fn create_client_handler() {
    let repo = Repository::new();
    let event_emitter = EventEmitter::new();
    let _: Client = Client::new(event_emitter, repo);
}

#[test]
fn client_handler_handles_request() {
    let repo = Repository::new();
    let emitter = EventEmitter::new();
    let mut handler = Client::new(emitter, repo);
    let _response = handler
        .handle_request(Request::now(Input::Ping, 0))
        .unwrap();
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
    let some_value = repo.get(key, std::time::SystemTime::now()).unwrap();
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
#[ignore = "reason"]
fn repository_is_not_updated_until_multi_is_commited() {
    todo!()
}

#[test]
fn repl_conf_returns_become_follower_command() {
    let repo = Repository::new();
    let emitter = EventEmitter::new();
    let mut replication_layer = ReplicationService {
        inner: MultiService::new(emitter, Hanlder::new(repo)),
    };
    let replconf = ReplConf::ListingPort(1);
    let replication_response: ReplicationResponse<Output> = replication_layer
        .call(Request::epoc(replconf.clone().into(), 0))
        .unwrap();
    assert_eq!(
        replication_response,
        ReplicationResponse::ReplicationRequest(replconf)
    );
}

#[test]
fn handler_returns_replconf_on_replconf() {
    let repo = Repository::new();
    let event_emitter = EventEmitter::new();
    let mut client: Client = Client::new(event_emitter, repo);
    let res = client
        .handle_request(Request::epoc(ReplConf::ListingPort(1).into(), 0))
        .unwrap();
    match res {
        Response::SendOutput(_) => panic!("expected replconf"),
        Response::RecivedReplconf(_) => (),
    }
}

#[test]
#[ignore = "todo"]
fn client_returns_into_follower_on_replconf() {
    todo!()
}

#[test]
fn event_layer_returns_none_for_ping() {
    let none_event = EventLayer::get_event(&Input::Ping);
    assert_eq!(none_event, None);
}

#[test]
fn event_layer_returns_none_for_get() {
    let none_event = EventLayer::get_event(&Input::Get(String::new()));
    assert_eq!(none_event, None);
}

#[test]
fn event_layer_returns_some_for_set() {
    let some_event = EventLayer::get_event(&Input::Set {
        key: "key".into(),
        value: "value".into(),
        expiry: None,
        get: false,
    });
    assert!(some_event.is_some());
}

#[test]
fn event_layer_returns_set_for_set() {
    let key = "key";
    let value = "value";
    let input = Input::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
        get: false,
    };
    let some_event = EventLayer::get_event(&input);
    let expected = event::Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
    };
    assert_eq!(some_event, Some(expected));
}

#[test]
fn event_layer_emits_event_from_get_event_on_call() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
    let mut event_layer = EventLayer::new(emitter, Hanlder::new(Repository::new()));
    let input = Input::Ping;
    let request = Request::now(input.clone(), 0);
    _ = event_layer.call(request);
    let event = subscriber.try_recive();
    let expected = EventLayer::get_event(&input);
    assert_eq!(event, expected);

    let input = Input::Set {
        key: "abc".into(),
        value: "xyz".into(),
        expiry: None,
        get: false,
    };
    let req = Request::now(input.clone(), 0);
    _ = event_layer.call(req);
    let event = subscriber.try_recive();
    let expected = EventLayer::get_event(&input);
    assert_eq!(event, expected);
}

#[test]
fn event_layer_gets_called() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
    let repo = Repository::new();
    let mut handler = Client::new(emitter, repo);

    let input = Input::Ping;
    _ = handler.handle_request(Request::now(input.clone(), 0));
    assert_eq!(subscriber.try_recive(), EventLayer::get_event(&input));

    let input = Input::Set {
        key: "abc".into(),
        value: "xyz".into(),
        expiry: None,
        get: false,
    };
    _ = handler.handle_request(Request::now(input.clone(), 0));
    assert_eq!(subscriber.try_recive(), EventLayer::get_event(&input));
}

#[test]
#[ignore = "todo"]
fn even_is_not_emitted_on_handler_panic() {
    todo!()
}
