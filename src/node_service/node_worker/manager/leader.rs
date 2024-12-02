use crate::node_service::{node_worker::Message, LeaderService};

pub struct LeaderManager {
    id: usize,
    tx: std::sync::mpsc::Sender<Message>,
    rx: std::sync::mpsc::Receiver<Message>,
}

impl LeaderService for LeaderManager {
    fn get_event_from_leader(&self) -> crate::node_service::node_worker::Kind {
        todo!()
    }
}

impl LeaderManager {
    #[must_use]
    pub fn new(
        id: usize,
        tx: std::sync::mpsc::Sender<Message>,
        rx: std::sync::mpsc::Receiver<Message>,
    ) -> Self {
        Self { id, tx, rx }
    }

    pub fn handle_replication_msg(&self) {
        todo!();
    }
}
