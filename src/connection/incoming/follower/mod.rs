use crate::{
    connection::{handshake::incoming::IncomingHandshake, Connection},
    event::Kind,
    resp::{Input, Message, Output, ReplConf},
};

#[cfg(test)]
mod tests;

pub struct Follower;

impl Follower {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event(&mut self, event: Kind) -> anyhow::Result<Option<Message>> {
        let res = match event {
            Kind::Set { key, value, expiry } => Some(Message::Input(crate::resp::Input::Set {
                key,
                value,
                expiry,
                get: false,
            })),
        };
        Ok(res)
    }

    pub fn handshake<C>(&mut self, connection: &mut C) -> anyhow::Result<()>
    where
        C: Connection,
    {
        let mut handshake = IncomingHandshake::new();
        while !handshake.is_finished() {
            let input = connection.read_message().unwrap().into_input().unwrap();
            let response = handshake.try_advance(&input).unwrap();
            connection.write_message(response.into()).unwrap();
        }
        Ok(())
    }
}
