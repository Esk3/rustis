use crate::connection::incoming::Follower;
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
    pub fn new() -> Self {
        todo!()
    }

    pub fn handle_follower_connection(mut self, mut input: Vec<resp::Value>) {
        tracing::info!("handling follower connection");
        let subscriber = self.emitter.subscribe();

        let mut handshake = crate::connection::handshake::incoming::IncomingHandshake::new();
        dbg!("starting handshake");
        while !handshake.is_finished() {
            let response = handshake.try_advance(&input).unwrap();
            dbg!("sending response", &response);
            self.connection.write(&response).unwrap();
            if handshake.is_finished() {
                break;
            }
            input = self.connection.read().unwrap().value.into_array().unwrap();
        }
        tracing::info!("handshake finished");
        let hex = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
        let data = hex::decode(hex).unwrap();
        let mut raw = b"$".to_vec();
        raw.extend(data.len().to_string().as_bytes());
        raw.extend(b"\r\n");
        raw.extend(data);
        let rdb = resp::Value::Raw(raw);
        self.connection.write(&rdb).unwrap();
        tracing::info!("rdb file sent");
        tracing::info!("connection with replica established sucesfully");

        let mut handler = Follower::new();
        loop {
            let event = subscriber.recive();
            let response = handler.handle_event(event).unwrap().unwrap();
            self.connection.write(&response).unwrap();
        }
    }
}
