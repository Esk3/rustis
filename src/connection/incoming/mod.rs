use anyhow::bail;
use client_connection::{client::response::ResponseKind, ClientConnectionResult};
use follower_connection::follower::Follower;
use tracing::instrument;

use crate::{connection::ConnectionError, event::EventEmitter, repository::Repository, resp};

pub mod client_connection;
mod follower_connection;

mod id {
    // TODO just pass id instead of using static
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    pub fn get_id() -> usize {
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

use id::get_id;

use super::stream::{PipelineBuffer, Stream};

#[cfg(test)]
pub mod tests;

pub struct IncomingConnection<S> {
    id: usize,
    connection: PipelineBuffer<S>,
    client_router: &'static client_connection::client::Router,
    repo: Repository,
    emitter: EventEmitter,
}

impl<S> IncomingConnection<S>
where
    S: Stream,
{
    #[must_use]
    pub fn new(
        stream: S,
        client_router: &'static client_connection::client::Router,
        emitter: EventEmitter,
        repo: Repository,
    ) -> Self {
        Self {
            id: get_id(),
            connection: PipelineBuffer::new(stream),
            client_router,
            repo,
            emitter,
        }
    }

    #[instrument(name = "incomming_connection_handler", skip(self), fields(connection.id = %self.id))]
    pub fn run_handler(mut self) -> anyhow::Result<()> {
        match self.handle_client_connection()? {
            ClientConnectionResult::Close => Ok(()),
            ClientConnectionResult::ReplicationMessage(messages) => {
                self.handle_follower_connection(messages);
                todo!();
                Ok(())
            }
        }
    }

    fn handle_client_connection(&mut self) -> anyhow::Result<ClientConnectionResult> {
        let mut client = client_connection::client::Client::new(
            self.client_router,
            self.emitter.clone(),
            self.repo.clone(),
        );
        let mut client = client_connection::ClientConnection::new(&mut self.connection, client);
        client.run();
        Ok(ClientConnectionResult::Close)
    }

    pub fn spawn_handler(self)
    where
        S: std::marker::Send + 'static,
    {
        std::thread::spawn(move || self.run_handler().unwrap());
    }

    fn handle_follower_connection(self, messages: Vec<resp::Value>) {
        todo!()
    }
}
