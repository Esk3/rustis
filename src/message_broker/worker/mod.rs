use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use crate::repository::{MemoryRepository, Repository};

use super::{manager::WorkerManager, message, Message};

mod kv_store;

#[derive(Debug)]
pub struct Worker<R = MemoryRepository> {
    repo: R,
    rx: Option<Receiver<Message<message::Request>>>,
    tx: Sender<Message<message::Request>>,
    managers: ManagerMailBoxes,
    subscribers: SubscriberMailBoxes,
}

impl<R> Worker<R>
where
    R: Repository,
{
    #[must_use]
    pub fn new(
        repository: R,
        tx: Sender<Message<message::Request>>,
        rx: Receiver<Message<message::Request>>,
    ) -> Self {
        Self {
            repo: repository,
            rx: Some(rx),
            tx,
            managers: ManagerMailBoxes::new(),
            subscribers: SubscriberMailBoxes::new(),
        }
    }
    pub fn run(mut self) {
        for message in self.rx.take().unwrap() {
            self.handle_message(message);
        }
    }
    #[must_use]
    pub fn spawn(repository: R) -> WorkerManager
    where
        R: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        let this = Self::new(repository, tx.clone(), rx);
        std::thread::spawn(move || {
            this.run();
        });

        WorkerManager::init(tx).unwrap()
    }

    pub fn get_message(&mut self) {}
    pub fn send_message(&mut self) {}
    pub fn handle_message(&mut self, Message { id, kind }: Message<message::Request>) {
        match self.handle_request(kind, id) {
            Ok(Some(response)) => self.managers.send(response, id),
            Ok(None) => (),
            Err(_) => todo!(),
        }
    }
    pub fn handle_request(
        &mut self,
        request: message::Request,
        id: usize,
    ) -> anyhow::Result<Option<message::Response>> {
        match request {
            message::Request::Join(tx) => {
                self.managers.join(tx);
                Ok(None)
            }
            message::Request::Subscribe => Ok(Some(self.subscribers.add())),
            message::Request::Get(key) => self.handle_get(&key),
            message::Request::Set {
                key,
                value,
                expiry,
                get,
            } => self
                .handle_set(key, value, expiry)
                .map(|res| if get { Some(res) } else { None }),
        }
    }
}

impl Worker<MemoryRepository> {
    #[must_use]
    pub fn spawn_with_memory_repository() -> WorkerManager {
        let repo = MemoryRepository::new();
        Self::spawn(repo)
    }
}

#[derive(Debug)]
struct ManagerMailBoxes {
    next_id: usize,
    managers: HashMap<usize, ManagerMailBox>,
}

impl ManagerMailBoxes {
    fn new() -> Self {
        Self {
            next_id: 0,
            managers: HashMap::new(),
        }
    }
    fn join(&mut self, tx: Sender<message::Response>) -> usize {
        self.next_id += 1;
        let id = self.next_id;
        tx.send(message::Response::Join(id)).unwrap();
        let manager = ManagerMailBox::new(tx, id);
        self.managers.insert(id, manager);
        id
    }
    fn send(&mut self, response: message::Response, id: usize) {
        let std::collections::hash_map::Entry::Occupied(entry) = self.managers.entry(id) else {
            return;
        };
        if entry.get().tx.send(response).is_err() {
            entry.remove();
        }
    }
}

#[derive(Debug)]
struct ManagerMailBox {
    id: usize,
    tx: Sender<message::Response>,
}

impl ManagerMailBox {
    fn new(tx: Sender<message::Response>, id: usize) -> Self {
        Self { id, tx }
    }
}

#[derive(Debug)]
struct SubscriberMailBoxes {
    next_id: usize,
    subscribers: Vec<SubscriberMailBox>,
}

impl SubscriberMailBoxes {
    fn new() -> Self {
        Self {
            next_id: 0,
            subscribers: Vec::new(),
        }
    }

    fn notify(&mut self, event: &message::Event) -> usize {
        self.subscribers
            .retain(|sub| sub.tx.send(event.clone()).is_ok());
        self.subscribers.len()
    }

    fn add(&mut self) -> message::Response {
        let (tx, rx) = std::sync::mpsc::channel();
        self.next_id += 1;
        self.subscribers
            .push(SubscriberMailBox::new(tx, self.next_id));
        message::Response::Subscribe {
            id: self.next_id,
            rx,
        }
    }
}

#[derive(Debug)]
struct SubscriberMailBox {
    id: usize,
    tx: Sender<message::Event>,
}
impl SubscriberMailBox {
    fn new(tx: Sender<message::Event>, id: usize) -> Self {
        Self { id, tx }
    }
}
