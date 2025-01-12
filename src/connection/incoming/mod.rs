use anyhow::bail;
use client::response::ResponseKind;
use follower::Follower;
use tracing::instrument;

use crate::{
    connection::{Connection, ConnectionError},
    event::EventEmitter,
    repository::Repository,
    resp,
};

pub mod client;
mod follower;

mod id {
    // TODO just pass id instead of using static
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    pub fn get_id() -> usize {
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

use id::get_id;

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
        connection: C,
        client_router: &'static client::Router,
        emitter: EventEmitter,
        repo: Repository,
    ) -> Self {
        Self {
            id: get_id(),
            connection,
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

    pub fn spawn_handler(self)
    where
        C: std::marker::Send + 'static,
    {
        std::thread::spawn(move || self.run_handler().unwrap());
    }

    fn handle_client_connection(&mut self) -> anyhow::Result<ClientConnectionResult> {
        tracing::info!("handling client connection");
        let mut client_handler =
            client::Client::new(self.client_router, self.emitter.clone(), self.repo.clone());
        let mut request_id = 0;
        loop {
            request_id += 1;
            match self
                .handle_client_request(&mut client_handler, request_id)
                .unwrap()
            {
                ClientRequestResult::Ok => (),
                ClientRequestResult::Close => todo!(),
                ClientRequestResult::ReplicationMessage(messages) => {
                    return Ok(ClientConnectionResult::ReplicationMessage(messages))
                }
            }
        }
    }

    #[instrument(name = "handle_client_request", skip(self, handler))]
    fn handle_client_request(
        &mut self,
        handler: &mut client::Client,
        request_id: usize,
    ) -> anyhow::Result<ClientRequestResult> {
        let requests = match self.connection.read_values() {
            Ok(request) => request,
            Err(ConnectionError::EndOfInput) => bail!("out of input"),
            Err(ConnectionError::Io(err)) => todo!("io err: {err}"),
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
            if let Some(event) = response.event {
                // TODO
                for event in event {
                    self.emitter.emmit(event);
                }
            }
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

        if replicate.is_empty() {
            Ok(ClientRequestResult::Ok)
        } else {
            // TODO
            tracing::warn!(
                "implementation incomplete some values may be dropped or consumed by client"
            );
            Ok(ClientRequestResult::ReplicationMessage(replicate))
        }
    }

    fn handle_follower_connection(mut self, mut inputs: Vec<resp::Value>) {
        tracing::info!("handling follower connection");
        let subscriber = self.emitter.subscribe();

        let mut handshake = crate::connection::handshake::incoming::IncomingHandshake::new();
        dbg!("starting handshake");
        while !handshake.is_finished() {
            if inputs.is_empty() {
                dbg!("reading new input");
                inputs.extend(
                    self.connection
                        .read_values()
                        .unwrap()
                        .into_iter()
                        .map(|v| v.value),
                );
            }
            for input in inputs.drain(..) {
                dbg!("handling input", &input);
                let response = handshake.try_advance(&input.into_array().unwrap()).unwrap();
                dbg!("sending response", &response);
                self.connection.write_values(vec![response]).unwrap();
            }
            ////let response = handshake.try_advance(&input.into_array().unwrap()).unwrap();
            //self.connection.write_values(responses).unwrap();
        }
        dbg!("handshake finished");
        let hex = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
        let data = hex::decode(hex).unwrap();
        let mut raw = b"$".to_vec();
        raw.extend(data.len().to_string().as_bytes());
        raw.extend(b"\r\n");
        raw.extend(data);
        let rdb = resp::Value::Raw(raw);
        self.connection.write_values(vec![rdb]).unwrap();
        dbg!("rdb file sent");

        let mut handler = Follower::new();
        loop {
            let event = subscriber.recive();
            let response = handler.handle_event(event).unwrap().unwrap();
            self.connection.write_values(vec![response]).unwrap();
        }
    }
}

enum ClientConnectionResult {
    Close,
    ReplicationMessage(Vec<resp::Value>),
}

enum ClientRequestResult {
    Ok,
    Close,
    ReplicationMessage(Vec<resp::Value>),
}
