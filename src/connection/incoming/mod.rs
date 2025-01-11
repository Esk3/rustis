use anyhow::bail;
use follower::Follower;
use tracing::{debug, info, instrument};

use crate::{
    connection::{Connection, ConnectionError},
    event::EventEmitter,
    repository::Repository,
    resp,
};

pub mod client;
mod follower;

//#[cfg(test)]
//pub mod tests;

pub struct IncomingConnection<C> {
    id: usize,
    connection: C,
    client_router: &'static client::Router,
    repo: Repository,
    emitter: EventEmitter,
}

impl<C> IncomingConnection<C>
where
    C: Connection,
{
    #[must_use]
    pub fn new(
        id: usize,
        connection: C,
        client_router: &'static client::Router,
        emitter: EventEmitter,
        repo: Repository,
    ) -> Self {
        Self {
            id,
            connection,
            client_router,
            repo,
            emitter,
        }
    }

    #[instrument(name = "incomming_connection_handler", skip(self), fields(connection.id = %self.id))]
    pub fn run_handler(mut self) -> anyhow::Result<()> {
        if self.handle_client_connection().is_ok() {
            self.handle_follower_connection();
        }
        Ok(())
    }

    pub fn spawn_handler(self)
    where
        C: std::marker::Send + 'static,
    {
        std::thread::spawn(move || self.run_handler());
    }

    fn handle_client_connection(&mut self) -> anyhow::Result<()> {
        info!("handling client connection");
        let mut client_handler =
            client::Client::new(self.client_router, self.emitter.clone(), self.repo.clone());
        let mut req_id = 0;
        loop {
            req_id += 1;
            let span = tracing::debug_span!("request", reqeust.id = %req_id);
            let _guard = span.enter();
            let request = match self.connection.read_value() {
                Ok(request) => request,
                Err(ConnectionError::EndOfInput) => bail!("out of input"),
                Err(ConnectionError::Io(_)) => todo!(),
                Err(ConnectionError::Any(err)) => {
                    tracing::warn!("err reading message: {err:?}");
                    todo!();
                    //self.connection.write_value(Output::Pong.into()).unwrap();
                    continue;
                }
            };
            tracing::trace!("handling request: {request:?}");
            let request = client::Request::now(request.value, request.bytes_read);
            let response = client_handler.handle_request(request).unwrap();
            tracing::trace!("got response: {response:?}");
            match response.kind {
                client::response::ResponseKind::Value(output) => {
                    self.connection.write_value(output).unwrap();
                }
                client::response::ResponseKind::RecivedReplconf(_) => return Ok(()),
            }
        }
    }

    fn handle_follower_connection(mut self) {
        info!("handling follower connection");
        let subscriber = self.emitter.subscribe();
        let mut handler = Follower::new();
        let event = subscriber.recive();
        let response = handler.handle_event(event).unwrap().unwrap();
        self.connection.write_value(response).unwrap();
    }
}
