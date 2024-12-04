use crate::node_service::{
    node_worker::{Kind, Message},
    FollowerService,
};

use super::ClientManager;

pub struct FollowerManager {
    id: usize,
    tx: std::sync::mpsc::Sender<Message>,
    rx: std::sync::mpsc::Receiver<Message>,
}

impl FollowerManager {
    pub(super) fn new(
        id: usize,
        tx: std::sync::mpsc::Sender<Message>,
        rx: std::sync::mpsc::Receiver<Message>,
    ) -> Self {
        Self { id, tx, rx }
    }
    #[must_use]
    pub fn get_event(&self) -> Kind {
        self.rx.recv().unwrap().kind
    }
}

impl FollowerService for FollowerManager {
    fn get_event_from_node(&self) -> Kind {
        let message = self.rx.recv().unwrap();
        message.kind
    }

    fn get_follower_byte_offset(&self) -> Kind {
        todo!()
    }
}

impl From<ClientManager> for FollowerManager {
    fn from(value: ClientManager) -> Self {
        value.into_follower()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_service::{node_worker::run, ClientService},
        repository::Repository,
    };

    use super::*;
    #[test]
    fn test() {
        let manager = run(crate::node::Node, Repository::new());
        let m2 = manager.clone();
        let follower = m2.into_follower();
        manager.set("abc".to_string(), "xyz".to_string()).unwrap();
        let msg = follower.get_event();
        assert_eq!(
            msg,
            Kind::ReplicateSet {
                key: "abc".to_string(),
                value: "xyz".to_string(),
                expiry: None
            }
        );
    }
}
