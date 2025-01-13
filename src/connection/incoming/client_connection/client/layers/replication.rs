use crate::{
    connection::incoming::client_connection::client,
    resp::{self, value::IntoRespArray},
    Service,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ReplicationResponse<T> {
    ReplicationRequest(resp::Value),
    Inner(T),
}

pub struct ReplicationService<S> {
    inner: S,
}

impl<S> ReplicationService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<client::Request> for ReplicationService<S>
where
    S: Service<client::Request, Error = anyhow::Error>,
{
    type Response = ReplicationResponse<S::Response>;

    type Error = anyhow::Error;

    fn call(&mut self, request: client::Request) -> Result<Self::Response, Self::Error> {
        if request
            .value
            .first()
            .unwrap()
            .eq_ignore_ascii_case("ReplConf")
        {
            return Ok(ReplicationResponse::ReplicationRequest(
                request.value.into_array(),
            ));
            todo!()
            //Ok(ReplicationResponse::ReplicationRequest(replconf)),
        } else if request.value.first().unwrap().eq_ignore_ascii_case("PSYNC") {
            todo!()
        } else {
            self.inner.call(request).map(ReplicationResponse::Inner)
        }
    }
}
