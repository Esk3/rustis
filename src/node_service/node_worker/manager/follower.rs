use crate::node_service::node_worker::Message;

pub struct FollowerManager {
    id: usize,
    tx: std::sync::mpsc::Sender<Message>,
    rx: std::sync::mpsc::Receiver<Message>,
}
