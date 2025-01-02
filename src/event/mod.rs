use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

#[cfg(test)]
pub mod tests;

pub type EventEmitter = LockEventProducer;
pub type EventSubscriber = ChannelEventSubscriber;

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

    pub fn emmit(&self, kind: Kind) {
        tracing::debug!("emitting event: {kind:?}");
        self.subscribers
            .lock()
            .unwrap()
            .iter()
            .for_each(|tx| tx.send(kind.clone()).unwrap());
    }

    #[must_use]
    pub fn subscribe(&self) -> EventSubscriber {
        let (tx, rx) = channel();
        self.subscribers.lock().unwrap().push(tx);
        EventSubscriber::new(rx)
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

    #[must_use]
    pub fn recive(&self) -> Kind {
        let event = self.rx.recv().unwrap();
        tracing::debug!("reciving event: {event:?}");
        event
    }

    #[must_use]
    pub fn try_recive(&self) -> Option<Kind> {
        let event = self.rx.try_recv().ok();
        tracing::debug!("try recive event: {event:?}");
        event
    }
}

impl IntoIterator for ChannelEventSubscriber {
    type Item = Kind;

    type IntoIter = std::sync::mpsc::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rx.into_iter()
    }
}
