use crate::node_service::FollowerService;

use super::response::Response;

pub struct Follower<S> {
    service: S,
    bytes_confirmed: usize,
}

impl<S> Follower<S>
where
    S: FollowerService,
{
    #[must_use]
    pub fn new(service: S) -> Self {
        Self {
            service,
            bytes_confirmed: 0,
        }
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
            crate::node_service::node_worker::Kind::Wait { count } => todo!(),
            crate::node_service::node_worker::Kind::WaitResponse { count } => todo!(),
            crate::node_service::node_worker::Kind::SyncBytesSent => Response::SyncBytesSent {
                bytes_confirmed: self.bytes_confirmed,
            },
            crate::node_service::node_worker::Kind::SyncBytesSentAck => todo!(),
            crate::node_service::node_worker::Kind::WaitTimeout => todo!(),
        }
    }

    fn handle_set<N>(&self, key: String, value: String, node: N) -> Result<(), ()> {
        todo!()
    }

    fn handle_wait<N>(&self, node: N) {
        if self.bytes_confirmed == self.service.get_follower_byte_offset() {
            todo!("connection should send ack to node");
        } else {
            assert!(
                self.bytes_confirmed < self.service.get_follower_byte_offset(),
                "the amount of bytes confirmed should always be less or equal to bytes sent"
            );
            todo!("connection should send network message to query bytes confirmed");
        }
        todo!()
    }
    pub fn send_wait_ack(&self) {
        self.service.wait_ack();
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
