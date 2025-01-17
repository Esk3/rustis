use event::EventEmitter;

use crate::message::request::Standard;

use super::*;

struct Test {
    leader: Leader,
    emitter: EventEmitter,
    repo: Repository,
}

impl Test {
    fn setup() -> Self {
        let repo = Repository::default();
        let emitter = EventEmitter::new();
        Self {
            leader: Leader::new(default_leader_router(), repo.clone()),
            emitter,
            repo,
        }
    }

    fn send_request(&mut self, request: Standard) -> anyhow::Result<LeaderResponse> {
        self.leader.handle_request(request.into())
    }

    fn send_request_assert_recive_none(&mut self, request: Standard) {
        assert_eq!(self.send_request(request).unwrap(), LeaderResponse::NONE);
    }
}

#[test]
fn leader_sets_and_emitts_event_silently() {
    let mut test = Test::setup();
    let subscriber = test.emitter.subscribe();
    let key = "MyKey";
    let value = "myValue";
    let request = Standard::new("SET", [key, value]);
    test.send_request_assert_recive_none(request);
    assert_eq!(
        test.repo.kv_repo().get(key, std::time::UNIX_EPOCH).unwrap(),
        Some(value.to_string())
    );
    assert_eq!(
        subscriber.try_recive().unwrap(),
        event::Kind::Set {
            key: key.to_string(),
            value: value.to_string(),
            expiry: None
        }
    );
}
