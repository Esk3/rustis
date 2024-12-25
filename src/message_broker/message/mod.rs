use std::sync::mpsc::{Receiver, Sender};

pub struct Message<T> {
    pub(crate) id: usize,
    pub(crate) kind: T,
}

impl<T> Message<T> {
    pub fn new(kind: T, id: usize) -> Self {
        Self { id, kind }
    }
}
pub enum Request {
    Join(Sender<Response>),
    Get(String),
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
        get: bool,
    },
    Subscribe,
}

impl Request {}
pub enum Response {
    Join(usize),
    Get(Option<String>),
    Set(Option<String>),
    Subscribe { id: usize, rx: Receiver<Event> },
}
#[derive(Debug, Clone)]
pub enum Event {
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    },
    GetAck,
}
