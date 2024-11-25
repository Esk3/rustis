use std::{collections::HashMap, sync};

use crate::{node::Node, repository::Repository};

use super::{request, response};

pub struct NodeWorker {
    node: Node,
    repo: Repository,
    rx: std::sync::mpsc::Receiver<super::Request>,
    clients: HashMap<usize, std::sync::mpsc::Sender<super::Response>>,
    followers: HashMap<usize, std::sync::mpsc::Sender<super::Response>>,
    next_id: usize,
}

impl NodeWorker {
    #[must_use]
    pub(super) fn new(
        node: Node,
        repo: Repository,
        rx: std::sync::mpsc::Receiver<super::Request>,
    ) -> Self {
        Self {
            node,
            repo,
            rx,
            clients: HashMap::new(),
            followers: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn run(mut self) {
        for super::Request { id, kind } in self.rx {
            let kind = match kind {
                request::Kind::Get { key } => {
                    let value =
                        Node::get(key, &mut self.repo, &std::time::SystemTime::now()).cloned();
                    response::Kind::Get { value }
                }
                request::Kind::Set { key, value, expiry } => {
                    Node::set(
                        key,
                        value,
                        &mut self.repo,
                        expiry,
                        &std::time::SystemTime::now(),
                    );
                    response::Kind::Set
                }
                request::Kind::NewConnection { tx } => {
                    if tx
                        .send(super::Response {
                            id: self.next_id,
                            kind: response::Kind::NewConnection { id: self.next_id },
                        })
                        .is_err()
                    {
                        continue;
                    }
                    self.clients.insert(self.next_id, tx);
                    self.next_id += 1;
                    continue;
                }
            };

            let response = super::Response { id, kind };
            Self::send_or_remove_response(id, response, &mut self.clients);
        }
    }

    fn send_or_remove_response(
        id: usize,
        response: super::Response,
        clients: &mut HashMap<usize, std::sync::mpsc::Sender<super::response::Response>>,
    ) {
        if let std::collections::hash_map::Entry::Occupied(entry) = clients.entry(id) {
            if let Err(_) = entry.get().send(response) {
                entry.remove();
            }
        }
    }
}
