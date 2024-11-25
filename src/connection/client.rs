use super::{response, Connection};
use crate::node_service::NodeService;

pub struct Client;

impl Client {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Connection for Client {
    fn handle_ping(&self) -> response::Ping {
        response::Ping::Pong
    }

    fn handle_echo(&self, echo: String) -> response::Echo {
        response::Echo::Echo(echo)
    }

    fn handle_get<N>(&self, key: String, node: N) -> response::Get<String>
    where
        N: NodeService,
    {
        todo!()
    }

    fn handle_set<N>(&self, key: String, value: String, node: N) -> response::Set
    where
        N: NodeService,
    {
        match node.set(key, value) {
            Ok(()) => response::Set::Ok,
            Err(()) => todo!(),
        }
    }

    fn handle_wait<N>(&self, node: N)
    where
        N: NodeService,
    {
        todo!()
    }
}
