mod common;
use core::panic;
use std::{io::Cursor, panic::catch_unwind};

use common::*;
use rustis::{
    connection::{client::ConnectionToClient, Connection, ConnectionWrapper},
    io::{Input, Io, Output},
    message_broker::{
        manager::{Manager, Subcriber},
        message,
        worker::Worker,
    },
    resp::Value,
};

#[test]
fn init_worker() {
    let _manager = Worker::spawn_with_memory_repository();
}

#[test]
fn into_subscriber() {
    let manager = Worker::spawn_with_memory_repository();
    let _subscriber = manager.get_subscriber().unwrap();
}

#[test]
fn subscriber_recives_events() {
    test_timeout(|| {
        let manager = Worker::spawn_with_memory_repository();
        let manager2 = manager.init_clone().unwrap();
        let subcriber = manager.get_subscriber().unwrap();
        let (key, value) = ("abc", "xyz");
        manager2
            .send(message::Request::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false,
            })
            .unwrap();
        subcriber.get_event().unwrap();
    });
}

#[test]
fn get_none() {
    let manager = Worker::spawn_with_memory_repository();
    manager.send(message::Request::Get("abc".into())).unwrap();
}
#[test]
fn set_value() {
    let manager = Worker::spawn_with_memory_repository();
}
#[test]
fn get_value() {}

#[test]
fn init_connection() {
    let manager = Worker::spawn_with_memory_repository();
    let mut input = Cursor::new(Vec::new());
    let mut output = Cursor::new(Vec::new());
    let io = Io::new(
        &mut input,
        &mut output,
        MockEncoder,
        MockParser::new([], []),
    );
    let _connection = Connection::new(TestService, io);
}

#[test]
fn get_request_on_connection() {
    //let manager = Worker::spawn_with_memory_repository();
    //let mut input = Cursor::new(Vec::new());
    //let mut output = Cursor::new(Vec::new());
    //let io = Io::new(
    //    &mut input,
    //    &mut output,
    //    MockEncoder,
    //    MockParser::new([Input::Ping], [Output::Pong]),
    //);
    //let mut connection = Connection::new(TestService, io);
    //connection.handle_client_request();
}

#[test]
fn connection_to_client() {
    let manager = Worker::spawn_with_memory_repository();
    let mut input = Cursor::new(Vec::new());
    let mut output = Cursor::new(Vec::new());
    let io = Io::new(
        &mut input,
        &mut output,
        MockEncoder,
        MockParser::new([Input::Ping], [Output::Pong]),
    );
    let client = ConnectionToClient::new_connection_to_client(manager, io);
    client.handle_client_request().unwrap();
}

#[test]
#[ignore = "reason"]
fn connection_to_follower() {
    todo!()
}

#[test]
#[should_panic(expected = "test timed out")]
fn connection_to_cient_into_connection_to_follower() {
    let manager = Worker::spawn_with_memory_repository();
    let input = Cursor::new(Vec::new());
    let output = Cursor::new(Vec::new());
    let io = Io::new(
        input,
        output,
        MockEncoder,
        MockParser::new(
            [
                Input::Ping,
                Input::ReplConf,
                Input::ReplConf,
                Input::Psync,
                Input::Ping,
            ],
            [
                Output::Pong,
                Output::ReplConf,
                Output::ReplConf,
                Output::Psync,
            ],
        ),
    );
    let client = ConnectionToClient::new_connection_to_client(manager, io);
    let mut connection_wrapper = ConnectionWrapper::Client(client);
    connection_wrapper = connection_wrapper.steps(4).unwrap();

    test_timeout(move || connection_wrapper.call()).unwrap();
}

#[test]
fn connection_to_follower_propegates_writes() {
    let manager = Worker::spawn_with_memory_repository();
    let input = Cursor::new(Vec::new());
    let output = input.clone();

    let io = Io::new(
        input.clone(),
        output.clone(),
        MockEncoder,
        MockParser::recive_handshake(),
    );
    let client = ConnectionToClient::new_connection_to_client(manager.init_clone().unwrap(), io);
    let mut follower = ConnectionWrapper::Client(client);
    follower = follower.steps(2).unwrap();

    let (key, value) = ("abc", "xyz");
    let client = ConnectionToClient::new_connection_to_client(
        manager,
        Io::new(
            input,
            output,
            MockEncoder,
            MockParser::new(
                [Input::Set {
                    key: key.into(),
                    value: value.into(),
                    expiry: None,
                    get: false,
                }],
                [Output::Set],
            ),
        ),
    );
    client.handle_client_request().unwrap();

    test_timeout(move || follower.call()).unwrap();
}
