use crate::node_service::{
    node_worker::{Kind, Message},
    LeaderService,
};

pub struct LeaderManager {
    id: usize,
    tx: std::sync::mpsc::Sender<Message>,
    rx: std::sync::mpsc::Receiver<Message>,
}

impl LeaderService for LeaderManager {
    fn get_event_from_leader(&self) -> crate::node_service::node_worker::Kind {
        todo!()
    }

    fn set(&self, key: String, value: String) -> Result<(), ()> {
        self.tx
            .send(Message {
                id: self.id,
                kind: crate::node_service::node_worker::Kind::Set {
                    key,
                    value,
                    expiry: None,
                },
            })
            .unwrap();
        let Message { id: _, kind } = self.rx.recv().unwrap();
        assert_eq!(kind, Kind::SetResponse);
        Ok(())
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
