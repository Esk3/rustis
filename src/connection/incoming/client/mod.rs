use layers::RoutingLayer;

use crate::{
    command::CommandRouter,
    event::{self, EventEmitter},
    repository::Repository,
    resp, Service,
};

pub mod commands;
pub mod layers;
pub mod request;
pub mod response;
pub mod router;

pub use request::Request;
pub use response::Response;
pub use router::{default_router, Router};

//#[cfg(test)]
//mod tests;

pub struct Client {
    service: layers::ReplicationService<layers::MultiLayer<RoutingLayer>>,
}

impl Client {
    pub fn new(router: &'static Router, emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            service: layers::ReplicationService::new(layers::MultiLayer::new(RoutingLayer::new(
                repo, router,
            ))),
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Response> {
        let res = self.service.call(request)?;
        let res = match res {
            layers::replication::ReplicationResponse::ReplicationRequest(value) => {
                Response::new(response::ResponseKind::RecivedReplconf(value), None)
            }
            layers::replication::ReplicationResponse::Inner(response) => response,
        };
        Ok(res)
    }
}
