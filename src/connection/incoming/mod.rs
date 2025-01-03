use anyhow::bail;
use client::{Client, ClientRequest};
use follower::Follower;
use tracing::{debug, info, instrument};

use crate::{
    connection::{Connection, ConnectionError, ConnectionMessage},
    event::EventEmitter,
    repository::Repository,
};

mod client;
mod follower;
#[cfg(test)]
pub mod tests;

pub struct IncomingConnection<C> {
    connection: C,
    repo: Repository,
    emitter: EventEmitter,
}

impl<C> IncomingConnection<C>
where
    C: Connection,
{
    #[must_use]
    pub fn new(connection: C, emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            connection,
            emitter,
            repo,
        }
    }

    #[instrument(skip(self))]
    pub fn handle_connection(mut self) -> anyhow::Result<()> {
        if self.handle_client_connection().is_ok() {
            self.handle_follower_connection();
        }
        Ok(())
    }

    fn handle_client_connection(&mut self) -> anyhow::Result<()> {
        info!("handling client connection");
        let mut client_handler = Client::new(self.emitter.clone(), self.repo.clone());
        loop {
            let request = match self.connection.read_message() {
                Ok(request) => request,
                Err(ConnectionError::EndOfInput) => bail!("out of input"),
            };
            debug!("handling request: {request:?}");
            let ConnectionMessage::Input(request) = request else {
                panic!();
            };
            let request = ClientRequest::now(request, 0);
            let response = client_handler.handle_request(request).unwrap();
            debug!("writing response: {response:?}");
            self.connection
                .write_message(ConnectionMessage::Output(response))
                .unwrap();
        }
    }

    fn handle_follower_connection(mut self) -> crate::event::Kind {
        info!("handling follower connection");
        let subscriber = self.emitter.subscribe();
        let mut handler = Follower::new();
        for event in subscriber {
            todo!()
        }
        todo!()
    }
}
