use crate::service::Service;

use super::Response;

#[cfg(test)]
mod tests;

pub struct ReplConf<S> {
    inner: S,
}

impl<S> ReplConf<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<crate::Request> for ReplConf<S>
where
    S: Service<crate::Request, Response = Response, Error = anyhow::Error>,
{
    type Response = Response;

    type Error = anyhow::Error;

    fn call(&mut self, request: crate::Request) -> Result<Self::Response, Self::Error> {
        if request.command().unwrap() == "REPLCONF" {
            todo!()
        } else {
            self.inner.call(request)
        }
    }
}

pub struct ResponseEater<S> {
    inner: S,
}

impl<S> ResponseEater<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<Req, S> Service<Req> for ResponseEater<S>
where
    S: Service<Req, Error = anyhow::Error>,
{
    type Response = Response;

    type Error = anyhow::Error;

    fn call(&mut self, request: Req) -> Result<Self::Response, Self::Error> {
        let _response = self.inner.call(request)?;
        Ok(Response::NoResponse)
    }
}
