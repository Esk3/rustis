use crate::{connection::incoming::client_connection::client, service::Service};

#[derive(Debug, PartialEq, Eq)]
pub enum ReplicationResponse<T> {
    ReplicationRequest(crate::Request),
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
    S: Service<client::Request>,
{
    type Response = ReplicationResponse<S::Response>;

    type Error = S::Error;

    fn call(&mut self, request: client::Request) -> Result<Self::Response, Self::Error> {
        if request.command().unwrap().eq_ignore_ascii_case("ReplConf") {
            Ok(ReplicationResponse::ReplicationRequest(request.request))
        } else if request.command().unwrap().eq_ignore_ascii_case("PSYNC") {
            todo!()
        } else {
            self.inner.call(request).map(ReplicationResponse::Inner)
        }
    }
}
