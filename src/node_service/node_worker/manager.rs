pub struct NodeManager {
    id: usize,
    tx: std::sync::mpsc::Sender<super::Request>,
    rx: std::sync::mpsc::Receiver<super::Response>,
}

impl crate::node_service::NodeService for NodeManager {
    fn get(&self, key: String) -> Result<String, ()> {
        self.tx
            .send(super::Request {
                id: self.id,
                kind: super::request::Kind::Get { key },
            })
            .unwrap();
        let value = self.rx.recv().unwrap();
        match value.kind {
            super::response::Kind::Get { value } => Ok(value),
        }
    }
}
