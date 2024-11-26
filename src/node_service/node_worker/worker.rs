use std::{clone, collections::HashMap};

use crate::{node::Node, repository::Repository};

use super::{Kind, Message};

pub struct NodeWorker {
    node: Node,
    repo: Repository,
    rx: std::sync::mpsc::Receiver<Message>,
    clients: HashMap<usize, std::sync::mpsc::Sender<Message>>,
    followers: HashMap<usize, std::sync::mpsc::Sender<Message>>,
    next_id: usize,
}

impl NodeWorker {
    #[must_use]
    pub(super) fn new(
        node: Node,
        repo: Repository,
        rx: std::sync::mpsc::Receiver<Message>,
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
        for Message { id, kind } in self.rx {
            let kind = match kind {
                Kind::Get { key } => {
                    let value =
                        Node::get(key, &mut self.repo, &std::time::SystemTime::now()).cloned();
                    Kind::GetResponse { value }
                }
                Kind::Set { key, value, expiry } => {
                    Node::set(
                        key.clone(),
                        value.clone(),
                        &mut self.repo,
                        expiry,
                        &std::time::SystemTime::now(),
                    );
                    for follower in self.followers.values() {
                        follower.send(Message {
                            id: 0,
                            kind: Kind::ReplicateSet {
                                key: key.clone(),
                                value: value.clone(),
                                expiry,
                            },
                        });
                    }
                    Kind::SetResponse
                }
                Kind::NewConnection { tx } => {
                    if tx
                        .send(Message {
                            id: self.next_id,
                            kind: Kind::NewConnectionResponse { id: self.next_id },
                        })
                        .is_err()
                    {
                        continue;
                    }
                    self.clients.insert(self.next_id, tx);
                    self.next_id += 1;
                    continue;
                }
                Kind::GetResponse { value } => todo!(),
                Kind::ReplicateSet { key, value, expiry } => todo!(),
                Kind::SetResponse => todo!(),
                Kind::NewConnectionResponse { id } => todo!(),
                Kind::ToFollower => {
                    let follower = self.clients.remove(&id).unwrap();
                    follower
                        .send(Message {
                            id,
                            kind: Kind::ToFollowerOk,
                        })
                        .unwrap();
                    self.followers.insert(id, follower);
                    continue;
                }
                Kind::ToFollowerOk => todo!(),
            };

            let response = Message { id, kind };
            Self::send_or_remove_response(id, response, &mut self.clients);
        }
    }

    fn send_or_remove_response(
        id: usize,
        message: Message,
        clients: &mut HashMap<usize, std::sync::mpsc::Sender<Message>>,
    ) {
        if let std::collections::hash_map::Entry::Occupied(entry) = clients.entry(id) {
            if let Err(_) = entry.get().send(message) {
                entry.remove();
            }
        }
    }
}
