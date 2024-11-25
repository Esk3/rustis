pub mod manager;
pub mod request;
pub mod response;
pub mod worker;

pub use request::Request;
pub use response::Response;

#[must_use]
pub fn run(node: crate::node::Node, repo: crate::repository::Repository) -> manager::NodeManager {
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = worker::NodeWorker::new(node, repo, rx);
    std::thread::spawn(|| worker.run());
    manager::NodeManager::join(tx)
}
