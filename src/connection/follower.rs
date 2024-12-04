use crate::node_service::FollowerService;

use super::response::Response;

pub struct Follower<S> {
    service: S,
}

impl<S> Follower<S>
where
    S: FollowerService,
{
    #[must_use]
    pub fn new(service: S) -> Self {
        Self { service }
    }

    pub fn get_event(&self) -> Response {
        let event = self.service.get_event_from_node();
        match event {
            crate::node_service::node_worker::Kind::Get { key } => todo!(),
            crate::node_service::node_worker::Kind::GetResponse { value } => todo!(),
            crate::node_service::node_worker::Kind::Set { key, value, expiry } => todo!(),
            crate::node_service::node_worker::Kind::ReplicateSet { key, value, expiry } => {
                Response::SendBulkString("TODO: resp encoded set".to_string())
            }
            crate::node_service::node_worker::Kind::SetResponse => todo!(),
            crate::node_service::node_worker::Kind::NewConnection { tx } => todo!(),
            crate::node_service::node_worker::Kind::NewConnectionResponse { id } => todo!(),
            crate::node_service::node_worker::Kind::ToFollower => todo!(),
            crate::node_service::node_worker::Kind::ToFollowerOk => todo!(),
        }
    }

    fn handle_set<N>(&self, key: String, value: String, node: N) -> Result<(), ()> {
        todo!()
    }

    fn handle_wait<N>(&self, node: N) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{node_service::node_worker::run, repository::Repository};

    use super::*;

    #[test]
    fn test() {
        let service = run(crate::node::Node, Repository::new()).into_follower();
        let follower = Follower::new(service);
        // ...
    }
}
