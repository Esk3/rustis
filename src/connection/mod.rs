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
    C: ClientService,
    F: FollowerService,
    L: LeaderService,
{
    #[must_use]
    pub fn new(kind: ConnectionKind<C, F, L>, io: IO) -> Self {
        Self { io, kind }
    }

    pub fn run(mut self) {
        loop {
            match self.kind {
                ConnectionKind::Client(ref mut client) => {
                    let Ok(request) = self.io.get_request() else {
                        dbg!("err");
                        break;
                    };
                    let response = client.handle_request(request);
                    self.io.send_response(response).unwrap();
                }
                ConnectionKind::Follower(ref follower) => {
                    let response = follower.get_event();
                    self.io.send_response(response).unwrap();
                }
                ConnectionKind::Leader(mut leader) => {
                    let request = self.io.get_request().unwrap();
                    let response = leader.handle_request();
                    todo!();
                }
            }
        }
    }
}

pub trait ConnectionInputOutput {
    fn get_request(&mut self) -> Result<request::Request, ()>;
    fn send_response(&mut self, response: response::Response) -> Result<(), ()>;
}

#[cfg(test)]
mod tests {
    use crate::{
        node_service::{
            node_worker::{
                manager::{ClientManager, FollowerManager, LeaderManager},
                run,
            },
            ClientService,
        },
        repository::Repository,
    };

    use super::{
        client::Client, request::Request, response::Response, Connection, ConnectionInputOutput,
    };

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
                let expected = responses.pop().unwrap();
                assert_eq!(response, expected);
            } else {
                dbg!(response);
            }
            Ok(())
        }
    }

    #[test]
    fn test() {
        let manager = run(crate::node::Node, Repository::new());
        let conn = Connection::<_, ClientManager, FollowerManager, LeaderManager>::new(
            super::wrapper::ConnectionKind::Client(Client::new(manager.clone())),
            MockIo::test_1(),
        );
        conn.run();
    }
    //use super::*;
    //
    //macro_rules! helper {
    //    ($conn:ty, $method:ident,  $expected:expr, $($y:expr),*) => {
    //        let result = $conn.$method($($y),*);
    //        assert_eq!(result, $expected);
    //    };
    //}
    //
    //mod ping {
    //    use crate::{connection::client, node_service::tests::dymmy_service::AlwaysOk};
    //
    //    macro_rules! ping_helper {
    //        (ping: $conn:ty, $expected:expr) => {
    //            helper!($conn, handle_ping, $expected,);
    //        };
    //        (ok: $conn:ty) => {
    //            ping_helper!(ping: $conn, Ping::Pong);
    //        };
    //        (null: $conn:ty) => {
    //            ping_helper!(ping: $conn, Ping::Null);
    //        };
    //    }
    //    #[test]
    //    fn client() {
    //        use crate::connection::response::Ping;
    //        ping_helper!(ok: client::Client<AlwaysOk>);
    //    }
    //    #[test]
    //    fn follower() {}
    //    #[test]
    //    fn leader() {}
    //}
    //
    //mod echo {
    //    use crate::{
    //        connection::{follower, leader, response::Echo},
    //        node_service::tests::dymmy_service::AlwaysOk,
    //    };
    //
    //    use super::client;
    //
    //    macro_rules! echo_helper {
    //        (echo: $conn:ty, $arg:expr, $expected:expr) => {
    //            helper!(
    //                $conn,
    //                handle_echo,
    //                $expected,
    //                $arg.to_string()
    //            );
    //        };
    //        (ok: $conn:ty, $arg:expr) => { echo_helper!(echo: $conn, $arg, Echo::Echo($arg.to_string()))};
    //        (null: $conn:ty, $arg:expr) => { echo_helper!(echo: $conn, $arg, Echo::Null($arg.to_string()))};
    //    }
    //
    //    #[test]
    //    fn client() {
    //        echo_helper!(ok: client::Client<AlwaysOk>, "hello world");
    //        echo_helper!(ok: client::Client<AlwaysOk>, "abc");
    //    }
    //
    //    #[test]
    //    fn follower() {
    //        echo_helper!(null: follower::Follower, "hello world");
    //        echo_helper!(null: follower::Follower, "abc");
    //    }
    //
    //    #[test]
    //    fn leader() {
    //        echo_helper!(null: leader::Leader, "hello world");
    //        echo_helper!(null: follower::Follower, "abc");
    //    }
    //}
    //
    //mod get {
    //    use crate::{
    //        connection::response,
    //        node_service::{self, tests::dymmy_service::AlwaysOk},
    //    };
    //
    //    use super::client;
    //
    //    #[test]
    //    fn ok_when_value_found() {
    //        let client = client::Client::<AlwaysOk>::new();
    //        let node = node_service::tests::dymmy_service::AlwaysOk;
    //        let response = client.handle_get("hello world".to_string());
    //        assert_eq!(
    //            response,
    //            response::Get::Value("dummy response for key hello world".to_string())
    //        );
    //    }
    //
    //    #[test]
    //    fn not_found_when_missing() {
    //        let client = client::Client::<AlwaysOk>::new();
    //        let node = node_service::tests::dymmy_service::NotFound;
    //        let response = client.handle_get("hello world".to_string());
    //        assert_eq!(response, response::Get::NotFound);
    //    }
    //}
    //
    //mod set {
    //    use crate::{
    //        connection::{client, response},
    //        node_service::tests::dymmy_service::{self, AlwaysOk},
    //    };
    //
    //    #[test]
    //    fn test_macro() {
    //        helper!(
    //            client::Client<AlwaysOk>,
    //            handle_set,
    //            response::Set::Ok,
    //            "something".to_string(),
    //            "other".to_string()
    //        );
    //    }
    //}
}
