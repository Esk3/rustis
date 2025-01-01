use super::*;

#[derive(Debug)]
pub struct DummyPanicSubscriber;
impl EventSubscriber for DummyPanicSubscriber {
    fn recive(&self) -> Kind {
        unimplemented!()
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
    type Subscriber = DummyPanicSubscriber;

    fn emmit(&self, _kind: crate::event::Kind) {}

    fn subscribe(&self) -> Self::Subscriber {
        DummyPanicSubscriber
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
    type Subscriber = DummyPanicSubscriber;

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
        assert!(
            self.0.lock().unwrap().is_empty(),
            "expected more events: {:?}",
            self.0.lock().unwrap()
        );
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
