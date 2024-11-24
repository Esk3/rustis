use super::{client::Client, follower::Follower, leader::Leader, response, Connection};
use crate::node_service::NodeService;

enum ConnectionKind {
    Client(Client),
    Follower(Follower),
    Leader(Leader),
}

impl Connection for ConnectionKind {
    fn handle_ping(&self) -> response::Ping {
        match self {
            ConnectionKind::Client(c) => c.handle_ping(),
            ConnectionKind::Follower(f) => f.handle_ping(),
            ConnectionKind::Leader(l) => l.handle_ping(),
        }
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
