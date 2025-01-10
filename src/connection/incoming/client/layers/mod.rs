use crate::{repository::Repository, resp, Service};

use super::{Response, Router};

//pub mod event;
pub mod multi;
pub mod replication;

pub use multi::MultiLayer;
pub use replication::ReplicationService;

//#[cfg(test)]
//mod tests;

pub struct RoutingLayer {
    pub repo: Repository,
    pub router: &'static Router,
}

impl RoutingLayer {
    pub fn new(repo: Repository, router: &'static Router) -> Self {
        Self { repo, router }
    }
}

impl Service<super::Request> for RoutingLayer {
    type Response = super::Response;

    type Error = anyhow::Error;

    fn call(&mut self, request: super::Request) -> Result<Self::Response, Self::Error> {
        let Some(handler) = self
            .router
            .route(request.value[0].clone().expect_string().unwrap().as_bytes())
        else {
            return Ok(Response::value(resp::Value::simple_string("not found")));
        };
        handler.call(request, &self.repo)
    }
}
