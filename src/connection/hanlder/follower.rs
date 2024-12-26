use std::fmt::Debug;

use tracing::instrument;

use crate::{
    event::{EventSubscriber, Kind},
    io::Output,
};

#[derive(Debug)]
pub struct FollowerState<E> {
    pub subscriber: E,
}

impl<E> FollowerState<E>
where
    E: EventSubscriber,
{
    pub fn new(event_reciver: E) -> Self {
        todo!()
    }
}

#[instrument]
pub fn handle_follower_event<E>(
    event: Kind,
    state: &mut FollowerState<E>,
) -> anyhow::Result<FollowerResult>
where
    E: EventSubscriber + Debug,
{
    tracing::debug!("handling event {event:?}");
    todo!()
}

pub enum FollowerResult {
    None,
    SendOutput(Output),
}
