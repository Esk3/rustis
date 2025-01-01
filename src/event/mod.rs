use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

#[cfg(test)]
pub mod tests;

pub trait EventProducer {
    type Subscriber: EventSubscriber;
    fn emmit(&self, kind: Kind);

    fn subscribe(&self) -> Self::Subscriber;
}

pub trait EventSubscriber {
    fn recive(&self) -> Kind;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Set {
        key: String,
        value: String,
        expiry: (),
    },
}

#[derive(Debug, Clone)]
pub struct LockEventProducer {
    subscribers: Arc<Mutex<Vec<Sender<Kind>>>>,
}

impl LockEventProducer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscribers: Arc::default(),
        }
    }
}

impl EventProducer for LockEventProducer {
    type Subscriber = ChannelEventSubscriber;
    fn emmit(&self, kind: Kind) {
        self.subscribers
            .lock()
            .unwrap()
            .iter()
            .for_each(|tx| tx.send(kind.clone()).unwrap());
    }

    fn subscribe(&self) -> Self::Subscriber {
        let (tx, rx) = channel();
        self.subscribers.lock().unwrap().push(tx);
        ChannelEventSubscriber::new(rx)
    }
}

#[derive(Debug)]
pub struct ChannelEventSubscriber {
    rx: Receiver<Kind>,
}
impl ChannelEventSubscriber {
    #[must_use]
    pub fn new(rx: Receiver<Kind>) -> Self {
        Self { rx }
    }
}

impl EventSubscriber for ChannelEventSubscriber {
    fn recive(&self) -> Kind {
        self.rx.recv().unwrap()
    }
}
