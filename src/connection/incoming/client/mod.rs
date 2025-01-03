use crate::{
    connection::{Input, Output},
    event::{self, EventEmitter},
    repository::Repository,
    Service,
};

#[cfg(test)]
mod tests;

pub struct ClientRequest {
    pub input: Input,
    pub input_length: usize,
    pub timestamp: std::time::SystemTime,
}

impl ClientRequest {
    pub fn now(input: Input, input_length: usize) -> Self {
        Self {
            input,
            input_length,
            timestamp: std::time::SystemTime::now(),
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

    pub fn handle_request(&mut self, request: ClientRequest) -> anyhow::Result<Output> {
        self.inner.call(request)
    }
}

struct ReplicationService {
    inner: MultiService,
}

impl Service<ClientRequest> for ReplicationService {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(
        &mut self,
        ClientRequest {
            input,
            input_length,
            timestamp,
        }: ClientRequest,
    ) -> Result<Self::Response, Self::Error> {
        match input {
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
            input => self.inner.call(ClientRequest {
                input,
                input_length,
                timestamp,
            }),
        }
    }
}

struct MultiService {
    inner: EventLayer,
    queue: Option<Vec<ClientRequest>>,
}

impl MultiService {
    fn new(emitter: EventEmitter, handler: Hanlder) -> Self {
        Self {
            inner: EventLayer::new(emitter, handler),
            queue: None,
        }
    }
}

impl Service<ClientRequest> for MultiService {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: ClientRequest) -> Result<Self::Response, Self::Error> {
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
                    queue.push(ClientRequest {
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
            Input::Ping => None,
            Input::Get(_) => None,
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

impl Service<ClientRequest> for EventLayer {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: ClientRequest) -> Result<Self::Response, Self::Error> {
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

impl Service<ClientRequest> for Hanlder {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(
        &mut self,
        ClientRequest {
            input,
            input_length,
            timestamp,
        }: ClientRequest,
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
            Input::Multi => todo!(),
            Input::CommitMulti => todo!(),
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
        };
        Ok(res)
    }
}
