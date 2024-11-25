pub struct NodeManager {
    id: usize,
    tx: std::sync::mpsc::Sender<super::Request>,
    rx: std::sync::mpsc::Receiver<super::Response>,
}

impl NodeManager {
    #[must_use]
    pub fn join(worker_tx: std::sync::mpsc::Sender<super::Request>) -> Self {
        let (manager_tx, manager_rx) = std::sync::mpsc::channel();
        let request = super::Request {
            id: 0,
            kind: super::request::Kind::NewConnection { tx: manager_tx },
        };
        worker_tx.send(request).unwrap();
        let res = manager_rx.recv().unwrap();
        match res.kind {
            crate::node_service::node_worker::response::Kind::NewConnection { id } => Self {
                id,
                tx: worker_tx,
                rx: manager_rx,
            },
            _ => todo!(),
        }
    }

    fn send(
        &self,
        kind: super::request::Kind,
    ) -> Result<(), std::sync::mpsc::SendError<super::Request>> {
        self.tx.send(super::Request { id: self.id, kind })
    }

    fn recive(&self) -> Result<super::response::Kind, std::sync::mpsc::RecvError> {
        self.rx.recv().map(|res| res.kind)
    }
}

impl crate::node_service::NodeService for NodeManager {
    fn get(&self, key: String) -> Result<Option<String>, ()> {
        self.tx
            .send(super::Request {
                id: self.id,
                kind: super::request::Kind::Get { key },
            })
            .unwrap();
        let value = self.rx.recv().unwrap();
        match value.kind {
            super::response::Kind::Get { value } => Ok(value),
            super::response::Kind::Set => todo!(),
            super::response::Kind::NewConnection { id } => todo!(),
        }
    }

    fn set(&self, key: String, value: String) -> Result<(), ()> {
        self.send(super::request::Kind::Set {
            key,
            value,
            expiry: None,
        })
        .unwrap();
        let Ok(super::response::Kind::Set) = self.recive() else {
            panic!();
        };
        Ok(())
    }

    fn wait(&self, count: usize) -> Result<(), ()> {
        todo!()
    }
}

impl Clone for NodeManager {
    fn clone(&self) -> Self {
        Self::join(self.tx.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_service::{node_worker, NodeService},
        repository::Repository,
    };

    use super::*;

    fn init() -> NodeManager {
        node_worker::run(crate::node::Node, Repository::new())
    }

    #[test]
    fn get_empty() {
        let manager = init();
        let value = manager.get("abc".to_string()).unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn get_value() {
        let manager = init();
        let key = "this is a key";
        let value = "this is a value";
        manager.set(key.to_string(), value.to_string()).unwrap();

        let result = manager.get(key.to_string()).unwrap();
        assert_eq!(result.unwrap(), value.to_string());
    }
}
