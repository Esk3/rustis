pub mod manager;
pub mod message;
pub mod worker;

pub use message::Kind;
pub use message::Message;

#[must_use]
pub fn run(node: crate::node::Node, repo: crate::repository::Repository) -> manager::ClientManager {
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = worker::NodeWorker::new(node, repo, rx);
    std::thread::spawn(|| worker.run());
    manager::ClientManager::join(tx)
}

#[cfg(test)]
mod tests {
    use crate::{
        node_service::{ClientService, LeaderService},
        repository::Repository,
    };

    use super::*;

    #[test]
    fn set_replicates_across_managers() {
        let m1 = run(crate::node::Node, Repository::new());
        let m2 = m1.clone();

        let key = "something";
        let value = "somevalue";
        assert!(m1.get(key.to_string()).unwrap().is_none());
        assert!(m2.get(key.to_string()).unwrap().is_none());

        m2.set(key.to_string(), value.to_string()).unwrap();

        assert_eq!(m1.get(key.to_string()).unwrap().unwrap(), value.to_string());
        assert_eq!(m1.get(key.to_string()).unwrap().unwrap(), value.to_string());
    }

    #[test]
    fn send_replicate_to_follower() {
        let manager = run(crate::node::Node, Repository::new());
        let none = manager.get("my key".to_string()).unwrap();
        assert!(none.is_none());
        let follower = manager.clone();
        let follower = follower.into_follower();
        manager
            .set("my key".to_string(), "my value".to_string())
            .unwrap();
        let e = follower.get_event();
        assert_eq!(
            e,
            Kind::ReplicateSet {
                key: "my key".to_string(),
                value: "my value".to_string(),
                expiry: None
            }
        );
        let value = manager.get("my key".to_string()).unwrap().unwrap();
        assert_eq!(value, "my value");
    }

    #[test]
    fn leader_replicates() {
        let manager = run(crate::node::Node, Repository::new());
        let leader = manager.clone().into_leader();
        leader.handle_replication_msg();
        let value = manager
            .get("same key as leader".to_string())
            .unwrap()
            .unwrap();
    }
}
