use crate::connection::incoming::client_connection::client::{self, Response};
use queue::Queue;

use crate::{
    resp::{self, value::IntoRespArray},
    Service,
};

pub mod queue;

#[cfg(test)]
mod tests;

pub struct MultiLayer<S> {
    inner: S,
    queue: Queue,
}

impl<S> MultiLayer<S>
where
    S: Service<client::Request, Response = Response, Error = anyhow::Error>,
{
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            queue: Queue::new(),
        }
    }
    fn commit_multi(&mut self, request: Vec<client::Request>) -> Response {
        // TODO:! currently the lock on the repo gets released after every call to `self.inner.call`
        // so another request could come and write to the repo in the middle of the transaction
        // which does not follow the redis protocol
        // https://redis.io/docs/latest/develop/interact/transactions/ (bullet point 1)
        let (values, events): (Vec<_>, Vec<_>) = request
            .into_iter()
            .map(|req| self.inner.call(req).unwrap())
            .map(|res| (res.value, res.events))
            .unzip();
        let events = events.into_iter().flatten().flatten().collect::<Vec<_>>();
        Response::value_events(values.into_array(), events)
    }
}

impl<S> Service<client::Request> for MultiLayer<S>
where
    S: Service<client::Request, Response = Response, Error = anyhow::Error>,
{
    type Response = Response;

    type Error = anyhow::Error;

    fn call(&mut self, request: client::Request) -> Result<Self::Response, Self::Error> {
        if self.queue.is_active() {
            return match self.queue.store(request) {
                queue::StoreResult::Ok => Ok(resp::Value::simple_string("QUEUED").into()),
                queue::StoreResult::InvalidStore(client::Request { request, .. }) => {
                    todo!("invalid store, {request:?}")
                }
                queue::StoreResult::QueueFinished(queue) => Ok(self.commit_multi(queue)),
            };
        }
        if request.command().unwrap().eq_ignore_ascii_case("MULTI") {
            self.queue.store(request);
            Ok(Response::ok())
        } else {
            self.inner.call(request)
        }
    }
}
