use client::Client;
use follower::Follower;
use leader::Leader;
use wrapper::ConnectionKind;

use crate::node_service::{ClientService, FollowerService, LeaderService};

pub mod client;
pub mod follower;
pub mod leader;
pub mod request;
pub mod response;
pub mod wrapper;

pub struct Connection<IO, C, F, L> {
    io: IO,
    kind: ConnectionKind<C, F, L>,
}

impl<IO, C, F, L> Connection<IO, C, F, L>
where
    IO: ConnectionInputOutput,
    C: ClientService<F = F>,
    F: FollowerService,
    L: LeaderService,
{
    #[must_use]
    pub fn new(kind: ConnectionKind<C, F, L>, io: IO) -> Self {
        Self { io, kind }
    }

    #[must_use]
    pub fn new_client(service: C, io: IO) -> Self {
        Self {
            io,
            kind: ConnectionKind::Client(Client::new(service)),
        }
    }

    #[must_use]
    pub fn connect_to_leader(service: L, mut io: IO) -> Self {
        // TODO: rename response to Message? more variants & check what to send
        io.send_response(response::Response::SendBulkString("PING".to_string()))
            .unwrap();
        io.send_response(response::Response::SendBulkString("SYNC".to_string()))
            .unwrap();
        Self {
            io,
            kind: ConnectionKind::Leader(Leader::new(service)),
        }
    }

    pub fn run(mut self) {
        while let Ok(this) = self.step() {
            self = this;
        }
    }

    pub fn step(mut self) -> Result<Self, ()> {
        match self.kind {
            ConnectionKind::Client(ref mut client) => {
                let Ok(request) = self.io.get_request() else {
                    #[cfg(not(test))]
                    dbg!("err");
                    return Err(());
                };
                match request {
                    request::Request::IntoFollower => {
                        self.kind = self.kind.into_follower();
                    }
                    request => {
                        let response = client.handle_request(request);
                        self.io.send_response(response).unwrap();
                    }
                }
            }
            ConnectionKind::Follower(ref follower) => {
                let response = follower.get_event();
                self.io.send_response(response).unwrap();
            }
            ConnectionKind::Leader(ref mut leader) => {
                let request = match self.io.get_request() {
                    Ok(request) => request,
                    Err(()) => {
                        return Err(());
                    }
                };
                let response = leader.handle_request(request);
                self.io.send_response(response).unwrap();
            }
        }
        Ok(self)
    }
}

pub trait ConnectionInputOutput {
    fn get_request(&mut self) -> Result<request::Request, ()>;
    fn send_response(&mut self, response: response::Response) -> Result<(), ()>;
}

#[cfg(test)]
mod tests {
    use std::char::ToLowercase;

    use crate::{
        node_service::{
            node_worker::{
                manager::{ClientManager, FollowerManager, LeaderManager},
                run,
            },
            tests::dummy_service::AlwaysOk,
            ClientService,
        },
        repository::Repository,
    };

    use super::{
        client::Client, request::Request, response::Response, Connection, ConnectionInputOutput,
    };

    #[derive(Debug)]
    struct MockIo {
        requests: Vec<Request>,
        responses: Option<Vec<Response>>,
    }

    impl MockIo {
        fn test_1() -> Self {
            Self::new_with_res(
                [
                    Request::Ping,
                    Request::Get("some key".to_string()),
                    Request::Set {
                        key: "some key".to_string(),
                        value: "some value".to_string(),
                        exp: None,
                    },
                    Request::Get("some key".to_string()),
                ],
                [
                    Response::SendPong,
                    Response::SendNull,
                    Response::SendOk,
                    Response::SendBulkString("some value".to_string()),
                ],
            )
        }
        fn new(requests: impl Into<Vec<Request>>) -> Self {
            Self {
                requests: requests.into().into_iter().rev().collect(),
                responses: None,
            }
        }
        fn new_with_res(
            requests: impl Into<Vec<Request>>,
            responses: impl Into<Vec<Response>>,
        ) -> Self {
            Self {
                requests: requests.into().into_iter().rev().collect(),
                responses: Some(responses.into().into_iter().rev().collect()),
            }
        }
    }
    impl ConnectionInputOutput for MockIo {
        fn get_request(&mut self) -> Result<super::request::Request, ()> {
            self.requests.pop().ok_or(()).inspect_err(|()| {
                self.responses.as_ref().inspect(|res| {
                    assert!(
                        res.is_empty(),
                        "expected more messages but input requests is empty {res:?}",
                    );
                });
            })
        }

        fn send_response(&mut self, response: super::response::Response) -> Result<(), ()> {
            if let Some(responses) = &mut self.responses {
                assert!(
                    !responses.is_empty(),
                    "got more responses than expected: {response:?}",
                );
                let expected = responses.pop().unwrap();
                match &expected {
                    Response::SendBulkString(s) if s == "*" => return Ok(()),
                    _ => (),
                }
                assert_eq!(response, expected, "{:?}", self);
            } else {
                dbg!(response);
            }
            Ok(())
        }
    }

    type MyConnection = Connection<MockIo, ClientManager, FollowerManager, LeaderManager>;

    fn setup(io: MockIo) -> (ClientManager, MyConnection)
where {
        let manager = run(crate::node::Node, Repository::new());
        let conn = Connection::new(
            super::wrapper::ConnectionKind::Client(Client::new(manager.clone())),
            io,
        );
        (manager, conn)
    }

    fn helper(io: MockIo) {
        let (_, conn) = setup(io);
        conn.run();
    }

    #[test]
    fn test() {
        let manager = run(crate::node::Node, Repository::new());
        let conn = Connection::<_, ClientManager, FollowerManager, LeaderManager>::new(
            super::wrapper::ConnectionKind::Client(Client::new(manager.clone())),
            MockIo::test_1(),
        );
        conn.run();
        let res = manager.get("some key".to_string()).unwrap().unwrap();
        assert_eq!(res, "some value");
    }

    #[test]
    fn test_2() {
        let (_manager, conn) = setup(MockIo::new_with_res(
            [Request::Echo("echo this".to_string())],
            [Response::SendBulkString("echo this".to_string())],
        ));
        conn.run();
    }

    #[test]
    fn test_queue() {
        helper(MockIo::new_with_res(
            [
                Request::Multi,
                Request::Get("k1".to_string()),
                Request::Set {
                    key: "k1".to_string(),
                    value: "v1".to_string(),
                    exp: None,
                },
                Request::Get("k1".to_string()),
                Request::ExecuteQueue,
                Request::Ping,
                Request::Get("k1".into()),
            ],
            [
                Response::SendOk,
                Response::SendBulkString("Queued".to_string()),
                Response::SendBulkString("Queued".to_string()),
                Response::SendBulkString("Queued".to_string()),
                Response::SendVec(vec![
                    Response::SendNull,
                    Response::SendOk,
                    Response::SendBulkString("v1".to_string()),
                ]),
                Response::SendPong,
                Response::SendBulkString("v1".into()),
            ],
        ));
    }

    #[test]
    fn test_wait() {
        helper(MockIo::new_with_res([Request::Wait], [Response::None]));
    }

    #[test]
    fn multiple_connections() {
        let manager = run(crate::node::Node, Repository::new());
        let conn1: MyConnection = Connection::new(
            super::wrapper::ConnectionKind::Client(Client::new(manager.clone())),
            MockIo::new_with_res(
                [
                    Request::Get("some key".to_string()),
                    Request::Set {
                        key: "some key".to_string(),
                        value: "some value".to_string(),
                        exp: None,
                    },
                ],
                [Response::SendNull, Response::SendOk],
            ),
        );
        let conn2: MyConnection = Connection::new(
            super::wrapper::ConnectionKind::Client(Client::new(manager.clone())),
            MockIo::new_with_res(
                [Request::Get("some key".to_string())],
                [Response::SendBulkString("some value".to_string())],
            ),
        );
        conn1.run();
        conn2.run();
    }

    #[test]
    #[should_panic(
        expected = "got more responses than expected: SendBulkString(\"TODO: resp encoded set\")"
    )]
    fn test_to_mock_follower() {
        let conn = Connection::new(
            crate::connection::wrapper::ConnectionKind::<AlwaysOk, AlwaysOk, AlwaysOk>::Client(
                Client::new(AlwaysOk),
            ),
            MockIo::new_with_res(
                [Request::IntoFollower],
                [Response::SendBulkString(
                    "TODO: resp encoded set".to_string(),
                )],
            ),
        );
        conn.run();
    }

    #[test]
    fn test_to_follower_replicates_threads() {
        let m = run(crate::node::Node, Repository::new());
        let mut conn1: MyConnection = Connection::new(
            super::wrapper::ConnectionKind::Client(Client::new(m.clone())),
            MockIo::new_with_res(
                [Request::IntoFollower],
                [Response::SendBulkString(
                    "TODO: resp encoded set".to_string(),
                )],
            ),
        );
        conn1 = conn1.step().unwrap();

        let h = std::thread::spawn(move || {
            conn1.step().unwrap();
        });

        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(!h.is_finished());

        let conn2: MyConnection = Connection::new(
            crate::connection::wrapper::ConnectionKind::Client(Client::new(m)),
            MockIo::new_with_res(
                [Request::Set {
                    key: "some key".to_string(),
                    value: "some value".to_string(),
                    exp: None,
                }],
                [Response::SendOk],
            ),
        );
        conn2.run();

        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(h.is_finished());
        h.join().unwrap();
    }

    #[test]
    fn test_follower_replicates_steps() {
        let m = run(crate::node::Node, Repository::new());
        let mut conn1 = MyConnection::new_client(
            m.clone(),
            MockIo::new_with_res(
                [Request::IntoFollower],
                [Response::SendBulkString("TODO: resp encoded set".into())],
            ),
        );
        let mut conn2 = MyConnection::new_client(
            m.clone(),
            MockIo::new_with_res(
                [Request::Set {
                    key: "a key".into(),
                    value: "a value".into(),
                    exp: None,
                }],
                [Response::SendOk],
            ),
        );
        conn1 = conn1.step().unwrap();

        conn2 = conn2.step().unwrap();

        conn1 = conn1.step().unwrap();
    }

    #[test]
    fn test_fake_leader_connection() {
        let m = run(crate::node::Node, Repository::new());
        let l = m.clone().into_leader();
        let mut conn1: MyConnection = Connection::connect_to_leader(
            l,
            MockIo::new_with_res(
                [Request::Set {
                    key: "anything".into(),
                    value: "any thing".into(),
                    exp: None,
                }],
                [
                    Response::SendBulkString("*".into()),
                    Response::SendBulkString("*".into()),
                    Response::None,
                ],
            ),
        );
        let mut conn2 = MyConnection::new_client(
            m,
            MockIo::new_with_res(
                [
                    Request::Get("anything".into()),
                    Request::Get("anything".into()),
                ],
                [
                    Response::SendNull,
                    Response::SendBulkString("any thing".into()),
                ],
            ),
        );
        conn2 = conn2.step().unwrap();
        conn1.run();
        conn2.run();
    }
    #[test]
    fn test_leader_connection() {
        let m = run(crate::node::Node, Repository::new());
        let l = m.clone().into_leader();
        let mut conn1: MyConnection =
            Connection::connect_to_leader(l, MockIo::new_with_res([], []));
        conn1.run();
    }

    #[test]
    fn multiple_client_connections_set_same_key() {
        let m = run(crate::node::Node, Repository::new());
        let key = "abc";
        let value = "xyz";
        let value2 = "123";
        let mut conn1 = MyConnection::new_client(
            m.clone(),
            MockIo::new_with_res(
                [
                    Request::Get(key.into()),
                    Request::Set {
                        key: key.into(),
                        value: value.into(),
                        exp: None,
                    },
                    Request::Get(key.into()),
                    Request::Get(key.into()),
                ],
                [
                    Response::SendNull,
                    Response::SendOk,
                    Response::SendBulkString(value.into()),
                    Response::SendBulkString(value2.into()),
                ],
            ),
        );
        let mut conn2 = MyConnection::new_client(
            m,
            MockIo::new_with_res(
                [
                    Request::Get(key.into()),
                    Request::Get(key.into()),
                    Request::Set {
                        key: key.into(),
                        value: value2.into(),
                        exp: None,
                    },
                    Request::Get(key.into()),
                ],
                [
                    Response::SendNull,
                    Response::SendBulkString(value.into()),
                    Response::SendOk,
                    Response::SendBulkString(value2.into()),
                ],
            ),
        );

        conn1 = conn1.step().unwrap();
        conn2 = conn2.step().unwrap();
        conn1 = conn1.step().unwrap();
        conn1 = conn1.step().unwrap();

        conn2 = conn2.step().unwrap();
        conn2 = conn2.step().unwrap();
        conn1.step().unwrap();
        conn2.run();
    }
}
