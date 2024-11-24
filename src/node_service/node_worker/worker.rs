use std::collections::HashMap;

use crate::{node::Node, repository::Repository};

pub struct NodeWorker {
    node: Node,
    repo: Repository,
    rx: std::sync::mpsc::Receiver<super::Request>,
    clients: HashMap<usize, std::sync::mpsc::Sender<super::response::Response>>,
}

impl NodeWorker {
    pub fn run(self) {
        for super::Request { id, kind } in self.rx {
            let response = match kind {
                super::request::Kind::Get { key } => {
                    Node::get(&key, &self.repo);
                    todo!()
                }
            };
            self.clients.get(&id).unwrap().send(response).unwrap();
        }
    }
}
