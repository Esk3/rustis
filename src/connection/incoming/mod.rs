use anyhow::bail;
use client::response::{self, ResponseKind};
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
        if let Ok(messages) = self.handle_client_connection() {
            self.handle_follower_connection(messages);
        }
        Ok(())
    }

    pub fn spawn_handler(self)
    where
        C: std::marker::Send + 'static,
    {
        std::thread::spawn(move || self.run_handler());
    }

    fn handle_client_connection(&mut self) -> anyhow::Result<Vec<resp::Value>> {
        info!("handling client connection");
        let mut client_handler =
            client::Client::new(self.client_router, self.emitter.clone(), self.repo.clone());
        let mut request_id = 0;
        loop {
            request_id += 1;
            let repl = self
                .handle_request(&mut client_handler, request_id)
                .unwrap();
            if !repl.is_empty() {
                return Ok(repl);
            }
        }
    }
    #[instrument(name="handle_client_request",skip(self, handler), fields(request.id=%request_id))]
    fn handle_request(
        &mut self,
        handler: &mut client::Client,
        request_id: usize,
    ) -> anyhow::Result<Vec<resp::Value>> {
        let requests = match self.connection.read_values() {
            Ok(request) => request,
            Err(ConnectionError::EndOfInput) => bail!("out of input"),
            Err(ConnectionError::Io(_)) => todo!(),
            Err(ConnectionError::Any(err)) => {
                tracing::warn!("err reading message: {err:?}");
                todo!();
                return Err(err);
            }
        };
        let mut responses = Vec::new();
        for request in requests {
            tracing::trace!("handling request: {request:?}");
            let request = client::Request::now(request.value, request.bytes_read);
            let response = handler.handle_request(request).unwrap();
            tracing::trace!("got response: {response:?}");
            responses.push(response.kind);
        }
        let mut output: Vec<resp::Value> = Vec::with_capacity(responses.len());
        let mut replicate = Vec::new();
        for response in responses {
            match response {
                ResponseKind::Value(response) => output.push(response),
                ResponseKind::RecivedReplconf(repl) => replicate.push(repl),
            }
        }
        self.connection.write_values(output).unwrap();
        //match response.kind {
        //    client::response::ResponseKind::Value(output) => {
        //        self.connection.write_value(output).unwrap();
        //    }
        //    client::response::ResponseKind::RecivedReplconf(_) => todo!(),
        //}
        Ok(replicate)
    }

    fn handle_follower_connection(mut self, input: Vec<resp::Value>) {
        info!("handling follower connection");
        let subscriber = self.emitter.subscribe();
        let mut handler = Follower::new();
        let event = subscriber.recive();
        let response = handler.handle_event(event).unwrap().unwrap();
        self.connection.write_values(vec![response]).unwrap();
    }
}
