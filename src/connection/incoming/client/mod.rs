use std::sync::Arc;

use layers::{MultiLayer, RoutingLayer};

use crate::{
    command::CommandRouter,
    event::{self, EventEmitter},
    repository::Repository,
    resp, Service,
};

pub mod commands;
//pub mod handler;
pub mod layers;

//#[cfg(test)]
//mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub value: Vec<resp::Value>,
    pub size: usize,
    pub timestamp: std::time::SystemTime,
}

impl Request {
    #[must_use]
    pub fn now(value: resp::Value, input_length: usize) -> Self {
        Self {
            value: value.into_array().unwrap(),
            size: input_length,
            timestamp: std::time::SystemTime::now(),
        }
    }
    #[allow(dead_code)]
    #[must_use]
    pub fn epoch(value: resp::Value, input_length: usize) -> Self {
        Self {
            value: value.into_array().unwrap(),
            size: input_length,
            timestamp: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResponseKind {
    Value(resp::Value),
    RecivedReplconf(resp::Value),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub kind: ResponseKind,
    pub event: Option<event::Kind>,
}

impl Response {
    pub fn into_output(self) -> Result<resp::Value, Self> {
        if let ResponseKind::Value(output) = self.kind {
            Ok(output)
        } else {
            Err(self)
        }
    }
}

pub struct ClientRouter(CommandRouter<Request, Response, Repository>);

impl ClientRouter {
    #[must_use]
    pub fn new() -> Self {
        Self(CommandRouter::new())
    }
    pub fn add<C>(&mut self, command: C) -> &mut CommandRouter<Request, Response, Repository>
    where
        C: crate::command::Command<Request, Response, Repository> + 'static,
    {
        self.0.add(command)
    }

    pub fn route(
        &self,
        cmd: &[u8],
    ) -> Option<&dyn crate::command::Command<Request, Response, Repository>> {
        self.0.route(cmd)
    }
}

pub struct Client {
    service: MultiLayer<RoutingLayer>,
}

impl Client {
    pub fn new(router: &'static ClientRouter, emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            service: MultiLayer {
                inner: RoutingLayer { repo, router },
            },
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Response> {
        self.service.call(request)
    }
}

#[must_use]
pub fn default_router() -> &'static ClientRouter {
    let mut router = ClientRouter::new();
    router
        .add(commands::ping::Ping)
        .add(commands::echo::Echo)
        .add(commands::get::Get)
        .add(commands::set::Set);
    Box::leak(Box::new(router))
}
