use crate::{
    event::EventEmitter,
    repository::Repository,
    resp::{Input, Output, ReplConf},
    Service,
};

mod commands;
pub mod handler;
pub mod layers;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub input: Input,
    pub input_length: usize,
    pub timestamp: std::time::SystemTime,
}

impl Request {
    pub fn now(input: Input, input_length: usize) -> Self {
        Self {
            input,
            input_length,
            timestamp: std::time::SystemTime::now(),
        }
    }
    #[allow(dead_code)]
    pub const fn epoch(input: Input, input_length: usize) -> Self {
        Self {
            input,
            input_length,
            timestamp: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    SendOutput(Output),
    RecivedReplconf(ReplConf),
}

impl Response {
    pub fn into_output(self) -> Result<Output, Self> {
        if let Self::SendOutput(output) = self {
            Ok(output)
        } else {
            Err(self)
        }
    }
}

pub struct Client {
    inner: layers::replication::ReplicationService,
}

impl Client {
    pub fn new(emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            inner: layers::replication::ReplicationService {
                inner: layers::multi::MultiLayer::new(emitter, handler::Hanlder::new(repo)),
            },
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Response> {
        self.inner.call(request).map(|res| match res {
            layers::replication::ReplicationResponse::ReplicationRequest(replconf) => {
                Response::RecivedReplconf(replconf)
            }
            layers::replication::ReplicationResponse::Inner(output) => Response::SendOutput(output),
        })
    }
}
