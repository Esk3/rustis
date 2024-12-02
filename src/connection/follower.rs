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
        todo!();
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
