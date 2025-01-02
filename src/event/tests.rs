use super::*;

#[test]
fn create_event_emitter() {
    let emitter = EventEmitter::new();
}

#[test]
fn create_event_subscriber() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
}

struct Test {
    emitter: EventEmitter,
    subscriber: EventSubscriber,
}

impl Test {
    fn setup() -> Self {
        let emitter = EventEmitter::new();
        Self {
            subscriber: emitter.subscribe(),
            emitter,
        }
    }
}

#[test]
fn emit_event() {
    let emitter = EventEmitter::new();
    emitter.emmit(Kind::Set {
        key: "key".to_string(),
        value: "value".to_string(),
        expiry: (),
    });
}

#[test]
fn subscriber_recives_emitted_event() {
    let Test {
        emitter,
        subscriber,
    } = Test::setup();
    let (key, value) = ("abc", "xyz");
    emitter.emmit(Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: (),
    });
    let event = subscriber.recive();
}

#[test]
#[ignore = "sleep"]
fn subscriber_blocks_when_no_event_is_avalible() {
    let Test {
        emitter,
        subscriber,
    } = Test::setup();
    let handle = std::thread::spawn(move || subscriber.recive());
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(!handle.is_finished());
}

#[test]
fn subscriber_recives_event_from_cloned_emitter() {
    let Test {
        emitter,
        subscriber,
    } = Test::setup();
    let clone = emitter.clone();
    let (key, value) = ("abc", "xyz");
    clone.emmit(Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: (),
    });
    let event = subscriber.recive();
}

#[test]
fn iter_events() {
    let Test {
        emitter,
        subscriber,
    } = Test::setup();
    let events = [
        Kind::Set {
            key: "123".into(),
            value: "abc".into(),
            expiry: (),
        },
        Kind::Set {
            key: "abc".into(),
            value: "123".into(),
            expiry: (),
        },
    ];
    events.iter().for_each(|e| emitter.emmit(e.clone()));
    drop(emitter);
    let recived_events = subscriber.into_iter().collect::<Vec<_>>();
    assert_eq!(recived_events, events);
}
