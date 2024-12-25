use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

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

#[derive(Debug)]
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

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use super::{EventProducer, EventSubscriber, Kind};

    pub struct MockPanicSubscriber;
    impl EventSubscriber for MockPanicSubscriber {
        fn recive(&self) -> Kind {
            todo!()
        }
    }

    pub struct MockEventSubscriber(Mutex<Vec<Kind>>);

    impl MockEventSubscriber {
        pub fn new<I>(events: I) -> Self
        where
            I: IntoIterator<Item = Kind>,
            <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
        {
            Self(Mutex::new(events.into_iter().rev().collect()))
        }
    }

    impl EventSubscriber for MockEventSubscriber {
        fn recive(&self) -> Kind {
            self.0
                .lock()
                .unwrap()
                .pop()
                .expect("got more events than expected")
        }
    }

    impl Drop for MockEventSubscriber {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }
            assert!(
                self.0.lock().unwrap().is_empty(),
                "got less events than expected"
            );
        }
    }

    #[test]
    #[should_panic(expected = "got less events than expected")]
    fn mock_subscriber_panics_on_too_few_events() {
        _ = MockEventSubscriber::new([Kind::Set {
            key: "abc".into(),
            value: "xyz".into(),
            expiry: (),
        }]);
    }

    #[test]
    #[should_panic(expected = "got more events than expected")]
    fn mock_subscriber_panics_on_too_many_events() {
        let sub = MockEventSubscriber::new([]);
        sub.recive();
    }

    #[test]
    #[should_panic(expected = "event kinds did not match")]
    #[ignore = "todo"]
    fn mock_subscriber_panics_on_wrong_kind_events() {
        todo!()
    }

    #[derive(Debug)]
    pub struct MockEventProducerSink;
    impl EventProducer for MockEventProducerSink {
        type Subscriber = MockPanicSubscriber;

        fn emmit(&self, _kind: crate::event::Kind) {}

        fn subscribe(&self) -> Self::Subscriber {
            todo!()
        }
    }

    #[derive(Debug)]
    pub struct MockEventProducer(std::sync::Mutex<Vec<Kind>>);

    impl MockEventProducer {
        #[must_use]
        pub fn new<I>(expected: I) -> Self
        where
            <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
            I: IntoIterator<Item = Kind>,
        {
            Self(std::sync::Mutex::new(expected.into_iter().rev().collect()))
        }
    }

    impl EventProducer for MockEventProducer {
        type Subscriber = MockPanicSubscriber;

        fn emmit(&self, kind: Kind) {
            let expected = self
                .0
                .lock()
                .unwrap()
                .pop()
                .expect("got more events than expected");
            assert_eq!(kind, expected);
        }

        fn subscribe(&self) -> Self::Subscriber {
            todo!()
        }
    }

    impl Drop for MockEventProducer {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }
            assert!(self.0.lock().unwrap().is_empty(), "expected more events");
        }
    }

    #[test]
    #[should_panic(expected = "expected more events")]
    fn mock_panics_on_too_few_events() {
        _ = MockEventProducer::new([Kind::Set {
            key: "abc".into(),
            value: "xyz".into(),
            expiry: (),
        }]);
    }
    #[test]
    #[should_panic(expected = "got more events than expected")]
    fn mock_panics_on_too_many_events() {
        let handler = MockEventProducer::new([]);
        handler.emmit(Kind::Set {
            key: "any".into(),
            value: "any".into(),
            expiry: (),
        });
    }
    #[test]
    #[should_panic(expected = "event kinds did not match")]
    #[ignore = "todo"]
    fn mock_panics_on_wrong_kind_events() {
        todo!()
    }
}
