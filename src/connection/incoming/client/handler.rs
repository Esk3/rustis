use crate::{
    repository::Repository,
    resp::{self, Input, Output},
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
            Input::XAdd {
                stream_key,
                entry_id,
                value,
            } => {
                let key = self
                    .repo
                    .stream_repo()
                    .xadd(stream_key, None, value)
                    .unwrap();
                Output::SimpleString(key.to_string())
            }
            Input::XRead => todo!(),
            Input::XRange {
                stream_key,
                start,
                end,
            } => {
                let values = self
                    .repo
                    .stream_repo()
                    .xrange(stream_key, start, end)
                    .unwrap();
                Output::Array(values.into_iter().map(Output::SimpleString).collect())
            }
            Input::Client => Output::Ok,
            Input::Config(_) => Output::Array(vec![
                Output::SimpleString("slave-read-only".into()),
                Output::SimpleString("yes".into()),
            ]),
            Input::Info => Output::Array(vec![
                Output::SimpleString("role:master".into()),
                Output::SimpleString(
                    "master_replid:a4abfedae71101d030054370c48f98c53b815c30".into(),
                ),
            ]),
        };
        Ok(res)
    }
}
