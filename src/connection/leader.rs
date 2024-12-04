use crate::node_service::LeaderService;

use super::{
    request::{self, Request},
    response::Response,
};

pub struct Leader<S> {
    service: S,
}

impl<S> Leader<S>
where
    S: LeaderService,
{
    #[must_use]
    pub fn new(service: S) -> Self {
        Self { service }
    }
    pub fn handle_request(&mut self, request: Request) -> Response {
        match request {
            Request::Ping => todo!(),
            Request::Echo(_) => todo!(),
            Request::Get(_) => todo!(),
            Request::Set { key, value, exp } => {
                self.service.set(key, value).unwrap();
            }
            Request::Info => todo!(),
            Request::Sync => todo!(),
            Request::IntoFollower => todo!(),
            Request::Wait => todo!(),
            Request::Multi => todo!(),
            Request::AbortQueue => todo!(),
            Request::ExecuteQueue => todo!(),
            Request::StreamAdd => todo!(),
            Request::StreamGet => todo!(),
            Request::StreamQuery => todo!(),
        }
        Response::None
    }
}
