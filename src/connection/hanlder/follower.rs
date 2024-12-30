use std::fmt::Debug;

use tracing::instrument;

use crate::{
    connection::Input,
    event::{EventSubscriber, Kind},
    repository::Repository,
};

#[derive(Debug)]
pub struct FollowerState<E, R> {
    subscriber: E,
    repo: R,
}

impl<E, R> FollowerState<E, R>
where
    E: EventSubscriber,
{
    pub fn new(subscriber: E, repo: R) -> Self {
        Self { subscriber, repo }
    }

    pub fn subscriber(&self) -> &E {
        &self.subscriber
    }
}

#[instrument(skip(state))]
pub fn handle_follower_event<E, R>(
    event: Kind,
    state: &mut FollowerState<E, R>,
) -> anyhow::Result<FollowerResult>
where
    E: EventSubscriber + Debug,
    R: Repository,
{
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
        event::tests::DummyPanicSubscriber,
        repository::{LockingMemoryRepository, Repository},
    };

    use super::{handle_follower_event, FollowerState};

    #[test]
    fn propegates_set_to_follower() {
        let (key, value) = ("abc", "xyz");
        let repo = LockingMemoryRepository::new();
        let mut state = FollowerState::new(DummyPanicSubscriber, repo.clone());
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
