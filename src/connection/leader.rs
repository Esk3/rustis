use super::{response, Connection};
use crate::node_service::NodeService;

pub struct Leader;
impl Connection for Leader {
    fn handle_ping(&self) -> response::Ping {
        todo!()
    }

    fn handle_echo(&self, echo: String) -> response::Echo {
        todo!()
    }

    fn handle_get<N>(&self, key: String, node: N) -> response::Get<String>
    where
        N: NodeService,
    {
        todo!()
    }

    fn handle_set<N>(&self, node: N)
    where
        N: NodeService,
    {
        todo!()
    }

    fn handle_wait<N>(&self, node: N)
    where
        N: NodeService,
    {
        todo!()
    }
}
