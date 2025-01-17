use client_connection::ClientConnectionResult;
use follower_connection::{follower::Follower, FollowerConnection};
use tracing::instrument;

use crate::{event::EventEmitter, repository::Repository};

pub mod client_connection;
mod follower_connection;

pub use error::Error;

use super::stream::{PipelineBuffer, Stream};

#[cfg(test)]
pub mod tests;

pub type Result<T> = std::result::Result<T, Error>;

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
        id: usize,
    ) -> Self {
        Self {
            id,

            connection: PipelineBuffer::new(stream),
            client_router,
            repo,
            emitter,
        }
    }

    #[instrument(name = "incomming_connection_handler", skip(self), fields(connection.id = %self.id))]
    pub fn run_handler(mut self) -> Result<()> {
        match self.handle_client_connection()? {
            ClientConnectionResult::Close => Ok(()),
            ClientConnectionResult::ReplicationMessage(messages) => {
                self.handle_follower_connection(messages)
            }
        }
    }

    fn handle_client_connection(&mut self) -> Result<ClientConnectionResult> {
        let client = client_connection::client::Client::new(self.client_router, self.repo.clone());
        let mut client = client_connection::ClientConnection::new(
            &mut self.connection,
            self.emitter.clone(),
            client,
        );
        Ok(client.run()?)
    }

    pub fn spawn_handler(self)
    where
        S: std::marker::Send + 'static,
    {
        std::thread::spawn(move || self.run_handler().unwrap());
    }

    fn handle_follower_connection(self, messages: crate::Request) -> Result<()> {
        let follower_connection = FollowerConnection::new(self.connection, self.emitter);
        Ok(follower_connection.run(messages)?)
    }
}

pub mod error {
    use super::{client_connection, follower_connection};

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("client connection error: {0}")]
        Client(#[from] client_connection::Error),
        #[error("follower connection error: {0}")]
        Follower(#[from] follower_connection::Error),
    }
}
