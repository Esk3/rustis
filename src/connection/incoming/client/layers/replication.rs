use crate::{
    connection::incoming::client,
    resp::{Input, Output, ReplConf},
    Service,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ReplicationResponse<T> {
    ReplicationRequest(ReplConf),
    Inner(T),
}

pub struct ReplicationService {
    pub inner: super::multi::MultiLayer,
}

impl Service<client::Request> for ReplicationService {
    type Response = ReplicationResponse<Output>;

    type Error = anyhow::Error;

    fn call(
        &mut self,
        client::Request {
            input,
            input_length,
            timestamp,
        }: client::Request,
    ) -> Result<Self::Response, Self::Error> {
        match input {
            Input::ReplConf(replconf) => Ok(ReplicationResponse::ReplicationRequest(replconf)),
            Input::Psync => todo!(),
            input => self
                .inner
                .call(client::Request {
                    input,
                    input_length,
                    timestamp,
                })
                .map(ReplicationResponse::Inner),
        }
    }
}
