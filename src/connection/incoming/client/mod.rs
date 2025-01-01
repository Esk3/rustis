use crate::{
    connection::{Input, Output},
    repository::Repository,
    Service,
};

#[cfg(test)]
mod tests;

pub struct Client {
    inner: ReplicationService,
}

impl Client {
    pub fn new(repo: Repository) -> Self {
        Self {
            inner: ReplicationService {
                inner: MultiService::new(repo),
            },
        }
    }

    pub fn handle_request(&mut self, request: Input) -> anyhow::Result<Output> {
        self.inner.call(request)
    }
}

struct ReplicationService {
    inner: MultiService,
}

impl Service<Input> for ReplicationService {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: Input) -> Result<Self::Response, Self::Error> {
        match request {
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
            _ => self.inner.call(request),
        }
    }
}

struct MultiService {
    inner: Hanlder,
    queue: Option<Vec<Input>>,
}

impl MultiService {
    fn new(repo: Repository) -> Self {
        Self {
            inner: Hanlder::new(repo),
            queue: None,
        }
    }
}

impl Service<Input> for MultiService {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: Input) -> Result<Self::Response, Self::Error> {
        if let Some(ref mut queue) = self.queue {
            let res = match request {
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
                    queue.push(req);
                    Output::Queued
                }
            };
            return Ok(res);
        }
        match request {
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

struct Hanlder {
    repo: Repository,
}

impl Hanlder {
    fn new(repo: Repository) -> Self {
        Self { repo }
    }
}

impl Service<Input> for Hanlder {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: Input) -> Result<Self::Response, Self::Error> {
        let res = match request {
            Input::Ping => Output::Pong,
            Input::Get(key) => {
                let value = self.repo.get(&key).unwrap();
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
