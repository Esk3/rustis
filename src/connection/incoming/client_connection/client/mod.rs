use crate::{repository::Repository, service::Service};

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

type ClientService = layers::ReplicationService<layers::MultiLayer<layers::Routing>>;

pub struct Client {
    service: ClientService,
}

impl Client {
    #[must_use]
    pub fn new(router: &'static Router, repo: Repository) -> Self {
        Self {
            service: layers::ReplicationService::new(layers::MultiLayer::new(
                layers::Routing::new(repo, router),
            )),
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Result> {
        tracing::debug!("handling request: {request:?}");
        let result = self.service.call(request)?;
        tracing::debug!("{result:?}");
        Ok(match result {
            layers::replication::ReplicationResponse::ReplicationRequest(value) => {
                Result::ReplicationMessage(value)
            }
            layers::replication::ReplicationResponse::Inner(response) => Result::Response(response),
        })
    }
}

#[derive(Debug)]
pub enum Result {
    Response(Response),
    ReplicationMessage(crate::Request),
}

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("command not found {0}")]
        NotFound(String),
        #[error("error parsing request")]
        Parsing,
        #[error("{0}")]
        Any(#[from] anyhow::Error),
    }
}
