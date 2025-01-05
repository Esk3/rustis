use crate::test_helper;

use super::*;

test_helper! {
    EventTest { emitter: EventEmitter, EventEmitter::new() }
    create_subscriber() {
        let _subscriber = emitter.subscribe();
    }
    emit_event() {
    emitter.emmit(Kind::Set {
        key: "key".to_string(),
        value: "value".to_string(),
        expiry: None,
    });
    };
    subscriber_recives_emitted_event() {
    let subscriber = emitter.subscribe();
    let (key, value) = ("abc", "xyz");
    emitter.emmit(Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
    });
    let _event = subscriber.recive();
};
    #[ignore = "sleep"]
    subscriber_blocks_when_no_event_is_avalible() {
    let subscriber = emitter.subscribe();
    let handle = std::thread::spawn(move || subscriber.recive());
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(!handle.is_finished());
};
    subscriber_recives_event_from_cloned_emitter() {
    let subscriber = emitter.subscribe();
    let clone = emitter.clone();
    let (key, value) = ("abc", "xyz");
    clone.emmit(Kind::Set {
        key: key.into(),
        value: value.into(),
        expiry: None,
    });
    let _event = subscriber.recive();
};

    iter_events() {
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
    events.iter().for_each(|e| emitter.emmit(e.clone()));
    drop(emitter);
    let recived_events = subscriber.into_iter().collect::<Vec<_>>();
    assert_eq!(recived_events, events);
};
}
