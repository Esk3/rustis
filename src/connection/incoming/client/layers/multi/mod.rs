use queue::Queue;

use crate::{
    connection::incoming::client::{self, handler::Hanlder},
    event::EventEmitter,
    resp::{Input, Output},
    Service,
};

use super::event::EventLayer;

pub mod queue;

#[cfg(test)]
mod tests;

pub struct MultiLayer {
    inner: EventLayer,
    queue: Queue,
}

impl MultiLayer {
    pub fn new(emitter: EventEmitter, handler: Hanlder) -> Self {
        Self {
            inner: EventLayer::new(emitter, handler),
            queue: Queue::new(),
        }
    }
    fn commit_multi(&mut self, request: Vec<client::Request>) -> Output {
        let arr = request
            .into_iter()
            .map(|req| self.inner.call(req).unwrap())
            .collect();
        Output::Array(arr)
    }
}

impl Service<client::Request> for MultiLayer {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: client::Request) -> Result<Self::Response, Self::Error> {
        if self.queue.is_active() {
            return match self.queue.store(request) {
                queue::StoreResult::Ok => Ok(Output::Queued),
                queue::StoreResult::InvalidStore(client::Request { input, .. }) => match input {
                    Input::Multi => Ok(Output::MultiError),
                    _ => todo!(),
                },
                queue::StoreResult::QueueFinished(queue) => Ok(self.commit_multi(queue)),
            };
        }
        match request.input {
            Input::Multi => match self.queue.store(request) {
                queue::StoreResult::Ok => Ok(Output::Multi),
                queue::StoreResult::InvalidStore(_) => Ok(Output::MultiError),
                queue::StoreResult::QueueFinished(_) => todo!(),
            },
            Input::CommitMulti => todo!(),
            _ => self.inner.call(request),
        }
    }
}
