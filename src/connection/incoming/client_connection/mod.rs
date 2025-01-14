use client::response::ResponseKind;
use tracing::instrument;

use crate::{
    connection::stream::{PipelineBuffer, Stream},
    event::EventEmitter,
    resp,
};

pub mod client;

pub struct ClientConnection<'a, S> {
    connection: &'a mut PipelineBuffer<S>,
    client: client::Client,
    emitter: EventEmitter,
}

impl<'a, S> ClientConnection<'a, S>
where
    S: Stream,
{
    pub fn new(
        connection: &'a mut PipelineBuffer<S>,
        emitter: EventEmitter,
        client: client::Client,
    ) -> Self {
        Self {
            connection,
            client,
            emitter,
        }
    }

    #[instrument(name = "client_connection", skip(self))]
    pub fn run(&mut self) -> anyhow::Result<ClientConnectionResult> {
        tracing::info!("handling client connection");
        let mut request_id = 0;
        loop {
            request_id += 1;
            match self.handle_client_request(request_id).unwrap() {
                ClientRequestResult::Ok => (),
                ClientRequestResult::Close => todo!(),
                ClientRequestResult::ReplicationMessage(messages) => {
                    return Ok(ClientConnectionResult::ReplicationMessage(messages))
                }
            }
        }
    }

    #[instrument(name = "handle_client_request", skip(self))]
    fn handle_client_request(&mut self, request_id: usize) -> anyhow::Result<ClientRequestResult> {
        let message = self.connection.read().unwrap();

        tracing::trace!("handling request: {message:?}");
        let request = client::Request::now(message.into());
        let response = self.client.handle_request(request).unwrap();
        tracing::trace!("got response: {response:?}");
        if let Some(event) = response.event {
            // TODO
            for event in event {
                self.emitter.emmit(event);
            }
        }

        let response = match response.kind {
            ResponseKind::Value(response) => response,
            ResponseKind::RecivedReplconf(repl) => {
                return Ok(ClientRequestResult::ReplicationMessage(repl))
            }
        };

        self.connection.write(&response).unwrap();
        Ok(ClientRequestResult::Ok)
    }
}

pub enum ClientConnectionResult {
    Close,
    ReplicationMessage(crate::Request),
}

pub enum ClientRequestResult {
    Ok,
    Close,
    ReplicationMessage(crate::Request),
}
