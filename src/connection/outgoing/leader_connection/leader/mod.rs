use crate::service::layers::command_router::Routeable;
use crate::service::Service;
use crate::{
    event::{self},
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
}

impl Leader {
    pub fn new(
        router: &'static crate::command::CommandRouter<Request, (), Repository>,
        repo: Repository,
    ) -> Self {
        let service = layers::ReplConf::new(layers::ResponseEater::new(
            service::layers::command_router::CommandRouter::new(repo, router),
        ));
        Self { service }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<LeaderResponse> {
        let result = match self.service.call(request) {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("{err}");
                return Ok(LeaderResponse::NONE);
            }
        };
        Ok(match result {
            Response::NoResponse => LeaderResponse::NONE,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LeaderResponse {
    pub value: Option<resp::Value>,
    pub events: Option<Vec<event::Kind>>,
}

impl LeaderResponse {
    pub const NONE: Self = Self::new(None, None);
    pub const fn new(value: Option<resp::Value>, events: Option<Vec<event::Kind>>) -> Self {
        Self { value, events }
    }
}

#[must_use]
pub fn default_leader_router(
) -> &'static crate::command::CommandRouter<crate::Request, (), Repository> {
    let mut router = crate::command::CommandRouter::new();
    router.add(commands::set::Set).add(commands::ping::Ping);
    Box::leak(Box::new(router))
}
