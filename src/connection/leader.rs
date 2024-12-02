use crate::node_service::LeaderService;

use super::{request, response::Response};

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
    pub fn handle_request(&mut self) -> Response {
        todo!();
    }
}
