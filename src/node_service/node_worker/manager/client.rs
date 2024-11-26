use crate::node_service::ClientService;

use super::super::{Kind, Message};

pub struct ClientManager {
    id: usize,
    tx: std::sync::mpsc::Sender<Message>,
    rx: std::sync::mpsc::Receiver<Message>,
}

impl ClientManager {
    #[must_use]
    pub fn join(worker_tx: std::sync::mpsc::Sender<Message>) -> Self {
        let (manager_tx, manager_rx) = std::sync::mpsc::channel();
        let request = Message {
            id: 0,
            kind: Kind::NewConnection { tx: manager_tx },
        };
        worker_tx.send(request).unwrap();
        let res = manager_rx.recv().unwrap();
        match res.kind {
            Kind::NewConnectionResponse { id } => Self {
                id,
                tx: worker_tx,
                rx: manager_rx,
            },
            _ => todo!(),
        }
    }

    #[must_use]
    pub fn into_follower(self) -> super::FollowerManager {
        self.send(Kind::ToFollower).unwrap();
        let response = self.recive().unwrap();
        assert_eq!(response, Kind::ToFollowerOk);
        super::FollowerManager::new(self.id, self.tx, self.rx)
    }

    fn send(&self, kind: Kind) -> Result<(), std::sync::mpsc::SendError<Message>> {
        self.tx.send(Message { id: self.id, kind })
    }

    fn recive(&self) -> Result<Kind, std::sync::mpsc::RecvError> {
        self.rx.recv().map(|res| res.kind)
    }
}

impl ClientService for ClientManager {
    fn get(&self, key: String) -> Result<Option<String>, ()> {
        self.tx
            .send(Message {
                id: self.id,
                kind: Kind::Get { key },
            })
            .unwrap();
        let value = self.rx.recv().unwrap();
        match value.kind {
            Kind::GetResponse { value } => Ok(value),
            Kind::SetResponse => todo!(),
            Kind::NewConnectionResponse { id } => todo!(),
            Kind::Get { key } => todo!(),
            Kind::Set { key, value, expiry } => todo!(),
            Kind::ReplicateSet { key, value, expiry } => todo!(),
            Kind::NewConnection { tx } => todo!(),
            Kind::ToFollower => todo!(),
            Kind::ToFollowerOk => todo!(),
        }
    }

    fn set(&self, key: String, value: String) -> Result<(), ()> {
        self.send(Kind::Set {
            key,
            value,
            expiry: None,
        })
        .unwrap();
        let Ok(Kind::SetResponse) = self.recive() else {
            panic!();
        };
        Ok(())
    }

    fn wait(&self, count: usize) -> Result<(), ()> {
        todo!()
    }
}

impl Clone for ClientManager {
    fn clone(&self) -> Self {
        Self::join(self.tx.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{node_service::node_worker, repository::Repository};

    use super::*;

    fn init() -> ClientManager {
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
