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
    fn handle_set<N>(&self, node: N)
    where
        N: NodeService;
    fn handle_wait<N>(&self, node: N)
    where
        N: NodeService;
}
