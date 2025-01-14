use super::*;

#[test]
fn create_subscriber() {
    let emitter = EventEmitter::new();
    let _subscriber = emitter.subscribe();
}
#[test]
fn emit_event() {
    let emitter = EventEmitter::new();
    emitter.emit(Kind::Set {
        key: "key".to_string(),
        value: "value".to_string(),
        expiry: None,
    });
}
#[test]
fn subscriber_recives_emitted_event() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
    let (key, value) = ("abc", "xyz");
    emitter.emit(Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
    });
    let _event = subscriber.recive();
}
#[ignore = "sleep"]
#[test]
fn subscriber_blocks_when_no_event_is_avalible() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
    let handle = std::thread::spawn(move || subscriber.recive());
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(!handle.is_finished());
}
#[test]
fn subscriber_recives_event_from_cloned_emitter() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
    let clone = emitter.clone();
    let (key, value) = ("abc", "xyz");
    clone.emit(Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
    });
    let _event = subscriber.recive();
}

#[test]
fn iter_events() {
    let emitter = EventEmitter::new();
    let subscriber = emitter.subscribe();
    let events = [
        Kind::Set {
            key: "123".into(),
            value: "abc".into(),
            expiry: None,
        },
        Kind::Set {
            key: "abc".into(),
            value: "123".into(),
            expiry: None,
        },
    ];
    events.iter().for_each(|e| emitter.emit(e.clone()));
    drop(emitter);
    let recived_events = subscriber.into_iter().collect::<Vec<_>>();
    assert_eq!(recived_events, events);
}
