use commands::CommandRouter;

use crate::{
    event::{self, EventEmitter},
    repository::Repository,
    resp,
};

pub mod commands;
//pub mod handler;
//pub mod layers;

//#[cfg(test)]
//mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub value: Vec<resp::Value>,
    pub size: usize,
    pub timestamp: std::time::SystemTime,
}

impl Request {
    pub fn now(value: resp::Value, input_length: usize) -> Self {
        Self {
            value: value.into_array().unwrap(),
            size: input_length,
            timestamp: std::time::SystemTime::now(),
        }
    }
    #[allow(dead_code)]
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

pub struct Client {
    router: &'static CommandRouter,
}

impl Client {
    pub fn new(router: &'static CommandRouter, emitter: EventEmitter, repo: Repository) -> Self {
        Self { router }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Response> {
        let Some(handler) = self
            .router
            .route(request.value[0].clone().expect_string().unwrap().as_bytes())
        else {
            return Ok(Response {
                kind: ResponseKind::Value(resp::Value::SimpleString("not found".into())),
                event: None,
            });
        };
        handler.handle(request)
    }
}

pub fn default_router() -> &'static CommandRouter {
    let mut router = CommandRouter::new();
    router.add(commands::ping::Ping).add(commands::echo::Echo);
    Box::leak(Box::new(router))
}
