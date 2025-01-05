use crate::{
    repository::Repository,
    resp::{Input, Output},
};

#[cfg(test)]
mod tests;

pub struct Handler {
    repo: Repository,
    bytes_processe: usize,
}

impl Handler {
    pub fn new(repo: Repository) -> Self {
        Self {
            repo,
            bytes_processe: 0,
        }
    }

    fn handle_request(&mut self, request: Input) -> anyhow::Result<Option<Output>> {
        self.add_processed_bytes(1);
        match request {
            Input::Ping => (),
            Input::Get(_) => todo!(),
            Input::Set {
                key,
                value,
                expiry,
                get: _,
            } => {
                self.repo.set(key, value, expiry)?;
            }
            Input::Multi => todo!(),
            Input::CommitMulti => todo!(),
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
        }
        Ok(None)
    }

    fn get_bytes_processed(&self) -> usize {
        self.bytes_processe
    }

    fn add_processed_bytes(&mut self, bytes: usize) {
        self.bytes_processe += bytes;
    }
}
