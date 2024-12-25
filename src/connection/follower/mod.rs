use crate::{
    io::Input,
    message_broker::{
        manager::{Manager, Subcriber, WorkerManager, WorkerSubscriber},
        message::Event,
    },
    service::Service,
};

use super::{Connection, ConnectionService};

pub type ConnectionToFollower = GetEventService<FollowerService>;

impl ConnectionToFollower {
    pub fn new(manager: WorkerManager) -> Self {
        let subscriber = manager.get_subscriber().unwrap();
        Self {
            subscriber,
            inner: FollowerService { manager },
        }
    }
}

pub enum Response {
    Ok,
    GetAck,
}

#[derive(Debug)]
pub struct GetEventService<S> {
    subscriber: WorkerSubscriber,
    inner: S,
}
impl<S> Service<()> for GetEventService<S> {
    type Response = Response;

    fn call(&mut self, request: ()) -> anyhow::Result<Self::Response> {
        let event = self.subscriber.get_event().unwrap();
        match event {
            Event::Set { key, value, expiry } => todo!(),
            Event::GetAck => Ok(Response::GetAck),
        }
    }
}

#[derive(Debug)]
pub struct FollowerService {
    manager: WorkerManager,
}
impl Service<()> for FollowerService {
    type Response = ();

    fn call(&mut self, request: ()) -> anyhow::Result<Self::Response> {
        todo!()
    }
}

//impl<R, W, E, P> ConnectionToFollower<R, W, E, P>
//where
//    R: std::io::Read,
//    W: std::io::Write,
//    E: Encoder,
//    P: Parser,
//{
//    pub fn new_connection_to_follower(manager: WorkerManager, io: Io<R, W, E, P>) -> Self {
//        let sub = manager.get_subscriber().unwrap();
//        Self {
//            service: FollowerService {
//                manager,
//                subscriber: sub,
//            },
//            io,
//        }
//    }
//
//    #[must_use]
//    pub fn handle_follower_event(mut self) -> Self {
//        self.service.call((), &mut self.io).unwrap();
//        self
//    }
//}

//#[derive(Debug)]
//pub struct FollowerService<M, S> {
//    manager: M,
//    subscriber: S,
//}
//
//impl<R, W, E, P, M> Service<(), R, W, E, P> for FollowerService<M, WorkerSubscriber>
//where
//    W: std::io::Write,
//    E: Encoder,
//    P: Parser,
//{
//    type Response = ();
//
//    fn call(&mut self, _request: (), io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response> {
//        let event = self.subscriber.get_event().unwrap();
//        dbg!(&event);
//        match event {
//            Event::Set { key, value, expiry } => io.write_input(Input::Set {
//                key,
//                value,
//                expiry,
//                get: false,
//            }),
//        };
//        todo!()
//    }
//}
