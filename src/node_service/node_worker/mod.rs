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
