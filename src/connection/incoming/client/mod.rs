use crate::{event::EventEmitter, repository::Repository, Service};

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
    service: layers::ReplicationService<layers::MultiLayer<layers::Routing>>,
}

impl Client {
    #[must_use]
    pub fn new(router: &'static Router, emitter: EventEmitter, repo: Repository) -> Self {
        let emitter = emitter;
        Self {
            service: layers::ReplicationService::new(layers::MultiLayer::new(
                layers::Routing::new(repo, router),
            )),
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Response> {
        tracing::debug!("handling request: {request:?}");
        let res = self.service.call(request)?;
        let res = match res {
            layers::replication::ReplicationResponse::ReplicationRequest(value) => {
                Response::new(response::ResponseKind::RecivedReplconf(value), None)
            }
            layers::replication::ReplicationResponse::Inner(response) => response,
        };
        tracing::debug!("{res:?}");
        Ok(res)
    }
}
