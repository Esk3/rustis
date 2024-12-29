use std::fmt::Debug;

use super::hanlder::{
    self,
    client::{handle_client_request, ClientResult, ClientState},
    follower::{handle_follower_event, FollowerState},
};

use thiserror::Error;
use tracing::instrument;

use crate::{
    event::{EventProducer, EventSubscriber},
    io::{IoError, MessageIo, NetworkMessage},
    repository::Repository,
    resp::parser::{Encode, Parse},
};

pub struct Incoming<P, En, I, E, R> {
    pd: std::marker::PhantomData<(P, En, I, E, R)>,
}

#[cfg(test)]
mod tests;

impl<P, En, I, E, R> Incoming<P, En, I, E, R>
where
    P: Parse,
    En: Encode,
    I: MessageIo + Debug,
    E: EventProducer + Debug,
    R: Repository + Debug,
    <E as EventProducer>::Subscriber: std::fmt::Debug,
{
    #[must_use]
    pub fn new(io: I, producer: E, repo: R) -> Self {
        todo!()
    }

    #[instrument(skip(self))]
    pub fn run(self) {}
}
