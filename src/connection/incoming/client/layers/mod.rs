use crate::{repository::Repository, resp, Service};

use super::{ClientRouter, Response, ResponseKind};

//pub mod event;
pub mod multi;
pub mod replication;

pub use multi::MultiLayer;
pub use replication::ReplicationService;

//#[cfg(test)]
//mod tests;

pub struct RoutingLayer {
    pub repo: Repository,
    pub router: &'static ClientRouter,
}

impl RoutingLayer {
    pub fn new(repo: Repository, router: &'static ClientRouter) -> Self {
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
            return Ok(Response {
                kind: ResponseKind::Value(resp::Value::SimpleString("not found".into())),
                event: None,
            });
        };
        handler.handle(request, &self.repo)
    }
}
