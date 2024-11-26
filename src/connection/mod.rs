use wrapper::ConnectionKind;

use crate::node_service::ClientService;

pub mod client;
pub mod follower;
pub mod leader;
pub mod request;
pub mod response;
pub mod wrapper;

pub struct Connection<IO, C> {
    io: IO,
    kind: ConnectionKind<C>,
}

impl<IO, C> Connection<IO, C>
where
    IO: ConnectionInputOutput,
    C: ClientService,
{
    #[must_use]
    pub fn new(kind: ConnectionKind<C>, io: IO) -> Self {
        Self { io, kind }
    }
    pub fn run(mut self) {
        match self.kind {
            ConnectionKind::Client(client) => {
                let request = self.io.get_request().unwrap();
                let response = match request {
                    request::Request::Ping => response::Response::Pong,
                    request::Request::Echo(_) => todo!(),
                    request::Request::Get(_) => todo!(),
                    request::Request::Set { .. } => todo!(),
                };
                self.io.send_response(response).unwrap();
            }
            ConnectionKind::Follower(_) => todo!(),
            ConnectionKind::Leader(_) => todo!(),
        }
    }
}

pub trait ConnectionInputOutput {
    fn get_request(&mut self) -> Result<request::Request, ()>;
    fn send_response(&mut self, response: response::Response) -> Result<(), ()>;
}

#[cfg(test)]
mod tests {
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
