use std::fmt::Debug;

use tracing::instrument;

use crate::{
    connection::Input,
    event::{EventEmitter, EventSubscriber, Kind},
    repository::Repository,
};

#[derive(Debug)]
pub struct FollowerState {
    subscriber: EventEmitter,
    repo: Repository,
}

impl FollowerState {
    pub fn new(subscriber: EventEmitter, repo: Repository) -> Self {
        Self { subscriber, repo }
    }

    pub fn subscriber(&self) -> &EventEmitter {
        &self.subscriber
    }
}

#[instrument(skip(state))]
pub fn handle_follower_event(
    event: Kind,
    state: &mut FollowerState,
) -> anyhow::Result<FollowerResult> {
    tracing::debug!("handling event {event:?}");
    let msg = match event {
        Kind::Set { key, value, expiry } => Input::Set {
            key,
            value,
            expiry: None,
            get: false,
        },
    };
    Ok(FollowerResult::SendToFollower(msg))
}

#[derive(Debug, PartialEq, Eq)]
pub enum FollowerResult {
    None,
    SendToFollower(Input),
}

#[cfg(test)]
mod tests {
    use crate::{
        connection::{hanlder::follower::FollowerResult, Input},
        event::EventEmitter,
        repository::{LockingMemoryRepository, Repository},
    };

    use super::{handle_follower_event, FollowerState};

    #[test]
    fn propegates_set_to_follower() {
        let (key, value) = ("abc", "xyz");
        let repo = Repository::new();
        let mut state = FollowerState::new(EventEmitter::new(), repo.clone());
        assert!(repo.get(key).unwrap().is_none());
        let FollowerResult::SendToFollower(msg) = handle_follower_event(
            crate::event::Kind::Set {
                key: key.into(),
                value: value.into(),
                expiry: (),
            },
            &mut state,
        )
        .unwrap() else {
            panic!();
        };
        assert_eq!(
            msg,
            Input::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false
            }
        );
        assert!(repo.get(key).unwrap().is_none());
    }
}
