use crate::{connection, repository::Repository, resp};

//#[cfg(test)]
//mod tests;

pub struct Request {
    input: resp::Value,
    bytes: usize,
}

impl Request {
    pub fn new(input: resp::Value, bytes: usize) -> Self {
        Self { input, bytes }
    }
}

impl TryFrom<connection::Value> for Request {
    type Error = anyhow::Error;

    fn try_from(
        connection::Value { value, bytes_read }: connection::Value,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            input: value,
            bytes: bytes_read,
        })
    }
}

pub struct Handler {
    repo: Repository,
    bytes_processed: usize,
}

impl Handler {
    pub fn new(repo: Repository) -> Self {
        Self {
            repo,
            bytes_processed: 0,
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Option<resp::Value>> {
        let bytes_processed = self.bytes_processed;
        self.add_processed_bytes(1);
        //match request.input {
        //    Input::Ping => (),
        //    Input::Get(_) => todo!(),
        //    Input::Set {
        //        key,
        //        value,
        //        expiry,
        //        get: _,
        //    } => {
        //        self.repo.set(key, value, expiry)?;
        //    }
        //    Input::Multi => todo!(),
        //    Input::CommitMulti => todo!(),
        //    Input::ReplConf(_) => {
        //        return Ok(Some(
        //            ReplConf::GetAck(bytes_processed.try_into().unwrap()).into(),
        //        ))
        //    }
        //    Input::Psync => todo!(),
        //    Input::XAdd { .. } => todo!(),
        //    Input::XRead => todo!(),
        //    Input::XRange {
        //        stream_key,
        //        start,
        //        end,
        //    } => todo!(),
        //    Input::Client => todo!(),
        //    Input::Config(_) => todo!(),
        //    Input::Info => todo!(),
        //}
        Ok(None)
    }

    pub fn get_bytes_processed(&self) -> usize {
        self.bytes_processed
    }

    pub fn add_processed_bytes(&mut self, bytes: usize) {
        self.bytes_processed += bytes;
    }
}
