use crate::{
    connection::{
        handshake::incoming::IncomingHandshake,
        stream::{PipelineBuffer, Stream},
    },
    event::Kind,
    resp::{self, value::IntoRespArray},
};

//#[cfg(test)]
//mod tests;

pub struct Follower;

impl Follower {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event(&mut self, event: Kind) -> anyhow::Result<Option<resp::Value>> {
        let res = match event {
            Kind::Set { key, value, expiry } => Some(
                vec![
                    resp::Value::bulk_string("SET"),
                    resp::Value::bulk_string(key),
                    resp::Value::bulk_string(value),
                ]
                .into_array(),
            ),
        };
        Ok(res)
    }

    pub fn handshake<S>(&mut self, connection: &mut PipelineBuffer<S>) -> anyhow::Result<()>
    where
        S: Stream,
    {
        let mut handshake = IncomingHandshake::new();
        while !handshake.is_finished() {
            let input = connection.read().unwrap().value;
            let response = handshake.try_advance(&input.into_array().unwrap()).unwrap();
            connection.write(&response).unwrap();
        }
        Ok(())
    }
}
