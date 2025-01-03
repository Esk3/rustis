use crate::{
    connection::{
        handshake::incoming::IncomingHandshake, Connection, ConnectionMessage, Input, Output,
        ReplConf,
    },
    event::Kind,
};

#[cfg(test)]
mod tests;

pub struct Follower;

impl Follower {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event(&mut self, event: Kind) -> anyhow::Result<Option<ConnectionMessage>> {
        let res = match event {
            Kind::Set { key, value, expiry } => {
                Some(ConnectionMessage::Input(crate::connection::Input::Set {
                    key,
                    value,
                    expiry,
                    get: false,
                }))
            }
        };
        Ok(res)
    }

    pub fn handshake<C>(&mut self, connection: &mut C) -> anyhow::Result<()>
    where
        C: Connection,
    {
        let mut handshake = IncomingHandshake::new();
        while !handshake.is_finished() {
            let response = connection.read_message().unwrap().into_input().unwrap();
            let msg = handshake.get_message().unwrap();
            handshake.handle_message_recived(response).unwrap();
            connection.write_message(msg.into()).unwrap();
        }
        Ok(())
    }
}
