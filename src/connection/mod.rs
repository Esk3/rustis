use super::node_service::NodeService;

pub mod client;
pub mod follower;
pub mod leader;
pub mod response;
pub mod wrapper;

pub trait Connection {
    fn handle_ping(&self) -> response::Ping;
    fn handle_echo(&self, echo: String) -> response::Echo;
    fn handle_get<N>(&self, key: String, node: N) -> response::Get<String>
    where
        N: NodeService;
    fn handle_set<N>(&self, key: String, value: String, node: N) -> response::Set
    where
        N: NodeService;
    fn handle_wait<N>(&self, node: N)
    where
        N: NodeService;
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! helper {
        ($conn:ty, $method:ident,  $expected:expr, $($y:expr),*) => {
            let conn = <$conn>::new();
            let result = conn.$method($($y),*);
            assert_eq!(result, $expected);
        };
    }

    mod ping {
        use crate::connection::client;

        macro_rules! ping_helper {
            (ping: $conn:ty, $expected:expr) => {
                helper!($conn, handle_ping, $expected,);
            };
            (ok: $conn:ty) => {
                ping_helper!(ping: $conn, Ping::Pong);
            };
            (null: $conn:ty) => {
                ping_helper!(ping: $conn, Ping::Null);
            };
        }
        #[test]
        fn client() {
            use crate::connection::response::Ping;
            use crate::connection::Connection;
            ping_helper!(ok: client::Client);
        }
        #[test]
        fn follower() {}
        #[test]
        fn leader() {}
    }

    mod echo {
        use crate::connection::{follower, leader, response::Echo};

        use super::{client, Connection};

        macro_rules! echo_helper {
            (echo: $conn:ty, $arg:expr, $expected:expr) => {
                helper!(
                    $conn,
                    handle_echo,
                    $expected,
                    $arg.to_string()
                );
            };
            (ok: $conn:ty, $arg:expr) => { echo_helper!(echo: $conn, $arg, Echo::Echo($arg.to_string()))};
            (null: $conn:ty, $arg:expr) => { echo_helper!(echo: $conn, $arg, Echo::Null($arg.to_string()))};
        }

        #[test]
        fn client() {
            echo_helper!(ok: client::Client, "hello world");
            echo_helper!(ok: client::Client, "abc");
        }

        #[test]
        fn follower() {
            echo_helper!(null: follower::Follower, "hello world");
            echo_helper!(null: follower::Follower, "abc");
        }

        #[test]
        fn leader() {
            echo_helper!(null: leader::Leader, "hello world");
            echo_helper!(null: follower::Follower, "abc");
        }
    }

    mod get {
        use crate::{connection::response, node_service};

        use super::{client, Connection};

        #[test]
        fn ok_when_value_found() {
            let client = client::Client::new();
            let node = node_service::tests::dymmy_service::AlwaysOk;
            let response = client.handle_get("hello world".to_string(), node);
            assert_eq!(
                response,
                response::Get::Value("dummy response for key hello world".to_string())
            );
        }

        #[test]
        fn not_found_when_missing() {
            let client = client::Client::new();
            let node = node_service::tests::dymmy_service::NotFound;
            let response = client.handle_get("hello world".to_string(), node);
            assert_eq!(response, response::Get::NotFound);
        }
    }

    mod set {
        use crate::{
            connection::{client, response, Connection},
            node_service::tests::dymmy_service,
        };

        #[test]
        fn test_macro() {
            helper!(
                client::Client,
                handle_set,
                response::Set::Ok,
                "something".to_string(),
                "other".to_string(),
                dymmy_service::AlwaysOk
            );
        }
    }
}
