use crate::service::layers::command_router::Routeable;
use crate::service::Service;
use crate::{
    event::{self, EventEmitter},
    repository::Repository,
    resp,
};
use crate::{service, Request};

mod commands;
mod layers;
mod response;

#[cfg(test)]
mod tests;

pub use response::Response;

type LeaderService = layers::ReplConf<
    layers::ResponseEater<service::layers::command_router::CommandRouter<Request, (), Repository>>,
>;

impl Routeable for Request {
    fn route_name(&self) -> Vec<u8> {
        self.command().unwrap().as_bytes().to_vec()
    }
}

pub struct Leader {
    service: LeaderService,
    repo: Repository,
    emitter: EventEmitter,
}

impl Leader {
    pub fn new(
        router: &'static crate::command::CommandRouter<Request, (), Repository>,
        emitter: EventEmitter,
        repo: Repository,
    ) -> Self {
        let service = layers::ReplConf::new(layers::ResponseEater::new(
            service::layers::command_router::CommandRouter::new(repo.clone(), router),
        ));
        Self {
            service,
            repo,
            emitter,
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<LeaderResponse> {
        let result = match self.service.call(request) {
            Ok(res) => res,
            Err(err) => {
                tracing::error!("{err:?}");
                tracing::warn!("ignoring error");
                return Ok(LeaderResponse {
                    value: None,
                    events: None,
                });
            }
        };
        Ok(match result {
            Response::NoResponse => LeaderResponse {
                value: None,
                events: None,
            },
        })
    }
}

#[derive(Debug)]
pub struct LeaderResponse {
    pub value: Option<resp::Value>,
    pub events: Option<Vec<event::Kind>>,
}

#[must_use]
pub fn default_leader_router(
) -> &'static crate::command::CommandRouter<crate::Request, (), Repository> {
    let mut router = crate::command::CommandRouter::new();
    router.add(commands::set::Set);
    Box::leak(Box::new(router))
}
