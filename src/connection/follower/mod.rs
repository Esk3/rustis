use crate::{
    io::{Encoder, Input, Io, Parser},
    message_broker::{
        manager::{Manager, Subcriber, WorkerManager, WorkerSubscriber},
        message::Event,
    },
    service::Service,
};

use super::Connection;

pub type ConnectionToFollower<R, W, E, P, M = WorkerManager, Sub = WorkerSubscriber> =
    Connection<R, W, E, P, FollowerService<M, Sub>>;

impl<R, W, E, P> ConnectionToFollower<R, W, E, P>
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    pub fn new_connection_to_follower(manager: WorkerManager, io: Io<R, W, E, P>) -> Self {
        let sub = manager.get_subscriber().unwrap();
        Self {
            service: FollowerService {
                manager,
                subscriber: sub,
            },
            io,
        }
    }

    #[must_use]
    pub fn handle_follower_event(mut self) -> Self {
        self.service.call((), &mut self.io).unwrap();
        self
    }
}

#[derive(Debug)]
pub struct FollowerService<M, S> {
    manager: M,
    subscriber: S,
}

impl<R, W, E, P, M> Service<(), R, W, E, P> for FollowerService<M, WorkerSubscriber>
where
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    type Response = ();

    fn call(&mut self, _request: (), io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response> {
        let event = self.subscriber.get_event().unwrap();
        dbg!(&event);
        match event {
            Event::Set { key, value, expiry } => io.write_input(Input::Set {
                key,
                value,
                expiry,
                get: false,
            }),
        };
        todo!()
    }
}
