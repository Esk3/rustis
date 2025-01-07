use crate::{
    repository::Repository,
    resp::{Input, Output},
    Service,
};
pub struct Hanlder {
    repo: Repository,
}

impl Hanlder {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }
}

impl Service<super::Request> for Hanlder {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(
        &mut self,
        super::Request {
            input,
            input_length: _,
            timestamp,
        }: super::Request,
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
                let value = self.repo.set(key, value, expiry).unwrap();
                if get {
                    Output::SetGet(value)
                } else {
                    Output::Set
                }
            }
            Input::Multi | Input::CommitMulti | Input::ReplConf(_) | Input::Psync => unreachable!(),
        };
        Ok(res)
    }
}
