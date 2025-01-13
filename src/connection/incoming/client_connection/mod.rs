use client::response::ResponseKind;
use tracing::instrument;

use crate::{
    connection::stream::{PipelineBuffer, Stream},
    resp,
};

pub mod client;

pub struct ClientConnection<'a, S> {
    connection: &'a mut PipelineBuffer<S>,
    client: client::Client,
}

impl<'a, S> ClientConnection<'a, S>
where
    S: Stream,
{
    pub fn new(connection: &'a mut PipelineBuffer<S>, client: client::Client) -> Self {
        Self { connection, client }
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
        let request = self.connection.read().unwrap();

        tracing::trace!("handling request: {request:?}");
        let request = client::Request::now(request.value, request.bytes_read);
        let response = self.client.handle_request(request).unwrap();
        tracing::trace!("got response: {response:?}");
        if let Some(event) = response.event {
            // TODO
            for event in event {
                //self.emitter.emmit(event);
                todo!()
            }
        }

        let response = match response.kind {
            ResponseKind::Value(response) => response,
            ResponseKind::RecivedReplconf(repl) => {
                return Ok(ClientRequestResult::ReplicationMessage(
                    repl.into_array().unwrap(),
                ))
            }
        };

        self.connection.write(&response).unwrap();
        Ok(ClientRequestResult::Ok)
    }
}

pub enum ClientConnectionResult {
    Close,
    ReplicationMessage(Vec<resp::Value>),
}

pub enum ClientRequestResult {
    Ok,
    Close,
    ReplicationMessage(Vec<resp::Value>),
}
