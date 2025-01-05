use crate::{
    event::{self, EventEmitter},
    repository::Repository,
    resp::{Input, Output, ReplConf},
    Service,
};

#[cfg(test)]
mod tests;

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
    pub const fn epoc(input: Input, input_length: usize) -> Self {
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
    inner: ReplicationService,
}

impl Client {
    pub fn new(emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            inner: ReplicationService {
                inner: MultiService::new(emitter, Hanlder::new(repo)),
            },
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Response> {
        self.inner.call(request).map(|res| match res {
            ReplicationResponse::ReplicationRequest(replconf) => {
                Response::RecivedReplconf(replconf)
            }
            ReplicationResponse::Inner(output) => Response::SendOutput(output),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ReplicationResponse<T> {
    ReplicationRequest(ReplConf),
    Inner(T),
}

struct ReplicationService {
    inner: MultiService,
}

impl Service<Request> for ReplicationService {
    type Response = ReplicationResponse<Output>;

    type Error = anyhow::Error;

    fn call(
        &mut self,
        Request {
            input,
            input_length,
            timestamp,
        }: Request,
    ) -> Result<Self::Response, Self::Error> {
        match input {
            Input::ReplConf(replconf) => Ok(ReplicationResponse::ReplicationRequest(replconf)),
            Input::Psync => todo!(),
            input => self
                .inner
                .call(Request {
                    input,
                    input_length,
                    timestamp,
                })
                .map(ReplicationResponse::Inner),
        }
    }
}

struct MultiService {
    inner: EventLayer,
    queue: Option<Vec<Request>>,
}

impl MultiService {
    fn new(emitter: EventEmitter, handler: Hanlder) -> Self {
        Self {
            inner: EventLayer::new(emitter, handler),
            queue: None,
        }
    }
}

impl Service<Request> for MultiService {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: Request) -> Result<Self::Response, Self::Error> {
        if let Some(ref mut queue) = self.queue {
            let res = match request.input {
                Input::Multi => Output::MultiError,
                Input::CommitMulti => {
                    let arr = self
                        .queue
                        .take()
                        .unwrap()
                        .into_iter()
                        .map(|req| self.inner.call(req).unwrap())
                        .collect();
                    Output::Array(arr)
                }
                req => {
                    queue.push(Request {
                        input: req,
                        input_length: request.input_length,
                        timestamp: request.timestamp,
                    });
                    Output::Queued
                }
            };
            return Ok(res);
        }
        match request.input {
            Input::Multi => {
                if self.queue.is_some() {
                    return Ok(Output::MultiError);
                }
                self.queue = Some(Vec::new());
                Ok(Output::Multi)
            }
            Input::CommitMulti => todo!(),
            _ => self.inner.call(request),
        }
    }
}

struct EventLayer {
    emitter: EventEmitter,
    handler: Hanlder,
}

impl EventLayer {
    fn new(emitter: EventEmitter, handler: Hanlder) -> Self {
        Self { emitter, handler }
    }

    fn get_event(input: &Input) -> Option<event::Kind> {
        match input {
            Input::Ping | Input::Get(_) => None,
            Input::Set {
                key,
                value,
                expiry,
                get: _,
            } => Some(event::Kind::Set {
                key: key.to_string(),
                value: value.to_string(),
                expiry: *expiry,
            }),
            Input::Multi => todo!(),
            Input::CommitMulti => todo!(),
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
        }
    }
}

impl Service<Request> for EventLayer {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: Request) -> Result<Self::Response, Self::Error> {
        let get_event = Self::get_event(&request.input);
        let result = self.handler.call(request);
        if let Some(event) = get_event {
            if result.is_ok() {
                self.emitter.emmit(event);
            }
        }
        result
    }
}

struct Hanlder {
    repo: Repository,
}

impl Hanlder {
    fn new(repo: Repository) -> Self {
        Self { repo }
    }
}

impl Service<Request> for Hanlder {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(
        &mut self,
        Request {
            input,
            input_length,
            timestamp,
        }: Request,
    ) -> Result<Self::Response, Self::Error> {
        let res = match input {
            Input::Ping => Output::Pong,
            Input::Get(key) => {
                let value = self.repo.get(&key, timestamp).unwrap();
                Output::Get(value)
            }
            Input::Set {
                key,
                value,
                expiry,
                get,
            } => {
                self.repo.set(key, value, expiry).unwrap();
                Output::Set
            }
            Input::Multi | Input::CommitMulti | Input::ReplConf(_) | Input::Psync => unreachable!(),
        };
        Ok(res)
    }
}
