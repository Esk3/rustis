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

    macro_rules! test_connection_method {
        ($conn:ty, $method:ident,  $expected:expr, $($y:expr),*) => {
            let conn = <$conn>::new();
            let result = conn.$method($($y),*);
            assert_eq!(result, $expected);
        };

        (echo: $conn:ty, $arg:expr, $expected:expr) => {
            test_connection_method!(
                    $conn,
                    handle_echo,
                    $expected,
                    $arg.to_string()
                );
        };
        (echo ok: $conn:ty, $arg:expr) => { test_connection_method!(echo: $conn, $arg, Echo::Echo($arg.to_string()))};
        (echo null: $conn:ty, $arg:expr) => { test_connection_method!(echo: $conn, $arg, Echo::Null($arg.to_string()))};

    }

    mod echo {
        use crate::{
            connection::{
                follower, leader,
                response::{self, Echo},
            },
            node_service::DummyService,
        };

        use super::{client, Connection};

        #[test]
        fn test_macro() {
            test_connection_method!(
                client::Client,
                handle_set,
                response::Set::Ok,
                "something".to_string(),
                "other".to_string(),
                DummyService
            );
        }

        #[test]
        fn client() {
            test_connection_method!(echo ok: client::Client, "hello world");
        }

        #[test]
        fn follower() {
            test_connection_method!(echo null: follower::Follower, "hello world");
        }

        #[test]
        fn leader() {
            test_connection_method!(echo null: leader::Leader, "hello world");
        }
    }
}
