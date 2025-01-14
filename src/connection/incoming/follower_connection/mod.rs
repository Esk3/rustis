use tracing::instrument;

use crate::connection::incoming::Follower;
use crate::event::EventSubscriber;
use crate::{
    connection::stream::{PipelineBuffer, Stream},
    event::EventEmitter,
    resp,
};

pub mod follower;

pub struct FollowerConnection<S> {
    connection: PipelineBuffer<S>,
    emitter: EventEmitter,
}

impl<S> FollowerConnection<S>
where
    S: Stream,
{
    pub fn new(connection: PipelineBuffer<S>, emitter: EventEmitter) -> Self {
        Self {
            connection,
            emitter,
        }
    }

    #[instrument(name = "follower_connection_handler", skip(self))]
    pub fn run(mut self, starting_input: crate::Request) -> anyhow::Result<()> {
        tracing::info!("handling follower connection");
        let subscriber = self.emitter.subscribe();

        self.handshake(starting_input).unwrap();
        tracing::info!("connection with replica established sucesfully");
        self.handle_events(&subscriber).unwrap();
        Ok(())
    }

    fn handle_events(&mut self, subscriber: &EventSubscriber) -> anyhow::Result<()> {
        let mut handler = Follower::new();
        loop {
            self.handle_event(subscriber, &mut handler).unwrap();
        }
    }

    fn handle_event(
        &mut self,
        subscriber: &EventSubscriber,
        handler: &mut Follower,
    ) -> anyhow::Result<()> {
        let event = subscriber.recive();
        let response = handler.handle_event(event).unwrap().unwrap();
        self.connection.write(&response).unwrap();
        Ok(())
    }

    fn handshake(&mut self, mut starting_input: crate::Request) -> anyhow::Result<()> {
        tracing::info!("starting handshake");
        let mut handshake = crate::connection::handshake::incoming::IncomingHandshake::new();

        while !handshake.is_finished() {
            let response = handshake.try_advance(&starting_input).unwrap();
            self.connection.write(&response).unwrap();
            if handshake.is_finished() {
                break;
            }
            starting_input = self.connection.read().unwrap().into();
        }

        tracing::debug!("sending rdb file");
        let hex = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
        let data = hex::decode(hex).unwrap();
        let mut raw = b"$".to_vec();
        raw.extend(data.len().to_string().as_bytes());
        raw.extend(b"\r\n");
        raw.extend(data);
        let rdb = resp::Value::Raw(raw);
        self.connection.write(&rdb).unwrap();

        tracing::info!("handshake complete");
        Ok(())
    }
}
