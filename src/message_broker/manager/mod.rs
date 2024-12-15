use std::sync::mpsc::{Receiver, Sender};

use anyhow::bail;

use super::{message, Message};

#[derive(Debug)]
pub struct WorkerManager {
    id: usize,
    worker: Sender<Message<message::Request>>,
    reciver: Receiver<message::Response>,
}

impl WorkerManager {
    pub fn init(worker: Sender<Message<message::Request>>) -> anyhow::Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        worker
            .send(Message::new(message::Request::Join(tx), 0))
            .unwrap();
        let id = rx.recv().unwrap();
        let message::Response::Join(id) = id else {
            bail!("expected join")
        };
        Ok(Self {
            id,
            worker,
            reciver: rx,
        })
    }
    pub fn init_clone(&self) -> anyhow::Result<Self> {
        Self::init(self.worker.clone())
    }
}

pub trait Manager {
    fn send_message(&self, message: Message<message::Request>) -> anyhow::Result<()>;
    fn send(&self, request: message::Request) -> anyhow::Result<()> {
        self.send_message(Message::new(request, self.get_id()))
    }
    fn recive(&self) -> anyhow::Result<message::Response>;
    fn get_id(&self) -> usize;
    fn get_subscriber(&self) -> anyhow::Result<WorkerSubscriber>;
}

impl Manager for WorkerManager {
    fn send_message(&self, message: Message<message::Request>) -> anyhow::Result<()> {
        Ok(self.worker.send(message)?)
    }

    fn recive(&self) -> anyhow::Result<message::Response> {
        Ok(self.reciver.recv()?)
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_subscriber(&self) -> anyhow::Result<WorkerSubscriber> {
        self.send(message::Request::Subscribe)?;
        let message::Response::Subscribe { id, rx } = self.recive()? else {
            bail!("expected subscribe");
        };
        Ok(WorkerSubscriber::new(rx, id))
    }
}

pub trait Subcriber {
    fn get_event(&self) -> anyhow::Result<message::Event>;
}

#[derive(Debug)]
pub struct WorkerSubscriber {
    id: usize,
    rx: Receiver<message::Event>,
}

impl WorkerSubscriber {
    fn new(rx: Receiver<message::Event>, id: usize) -> Self {
        Self { id, rx }
    }
}

impl Subcriber for WorkerSubscriber {
    fn get_event(&self) -> anyhow::Result<message::Event> {
        Ok(self.rx.recv()?)
    }
}
