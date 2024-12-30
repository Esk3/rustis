use client::ClientHandler;
use tracing::{debug, info, instrument};

use crate::connection::{Connection, ConnectionError, ConnectionMessage};

mod client;
#[cfg(test)]
mod tests;

pub struct IncomingConnection<C> {
    connection: C,
}

impl<C> IncomingConnection<C>
where
    C: Connection,
{
    #[must_use]
    pub fn new(connection: C) -> Self {
        Self { connection }
    }

    #[instrument(skip(self))]
    pub fn handle_connection(mut self) -> anyhow::Result<()> {
        info!("handling new connection");
        let mut client_handler = ClientHandler::new();
        loop {
            let request = match self.connection.read_command() {
                Ok(request) => request,
                Err(ConnectionError::EndOfInput) => return Ok(()),
            };
            debug!("handling request: {request:?}");
            let ConnectionMessage::Input(request) = request else {
                panic!();
            };
            let response = client_handler.handle_request(request);
            debug!("writing response: {response:?}");
            self.connection
                .write_command(ConnectionMessage::Output(response))
                .unwrap();
        }
    }
}
