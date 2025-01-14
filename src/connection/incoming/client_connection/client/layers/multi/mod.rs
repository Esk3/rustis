use crate::connection::incoming::client_connection::client;
use crate::connection::incoming::client_connection::client::Response;
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
        let map = request.into_iter().map(|req| self.inner.call(req).unwrap());
        let mut values = Vec::new();
        let mut events = Vec::new();
        for i in map {
            match i.kind {
                client::response::ResponseKind::Value(value) => values.push(value),
                client::response::ResponseKind::RecivedReplconf(_) => todo!(),
            }
            if let Some(event) = i.event {
                events.extend(event);
            }
        }
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
                queue::StoreResult::Ok => Ok(resp::Value::simple_string("Queued").into()),
                queue::StoreResult::InvalidStore(client::Request { request, .. }) => {
                    todo!("invalid store, {request:?}")
                }
                queue::StoreResult::QueueFinished(queue) => Ok(self.commit_multi(queue)),
            };
        }
        if request.command().unwrap().eq_ignore_ascii_case("MULTI") {
            self.queue.store(request);
            todo!("multi started msg")
        } else {
            self.inner.call(request)
        }
    }
}
