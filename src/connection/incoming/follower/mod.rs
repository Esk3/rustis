use crate::{
    connection::{handshake::incoming::IncomingHandshake, Connection},
    event::Kind,
    resp,
};

//#[cfg(test)]
//mod tests;

pub struct Follower;

impl Follower {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event(&mut self, event: Kind) -> anyhow::Result<Option<resp::Value>> {
        //let res = match event {
        //    Kind::Set { key, value, expiry } => Some(Message::Input(crate::resp::Input::Set {
        //        key,
        //        value,
        //        expiry,
        //        get: false,
        //    })),
        //};
        //Ok(res)
        todo!()
    }

    pub fn handshake<C>(&mut self, connection: &mut C) -> anyhow::Result<()>
    where
        C: Connection,
    {
        let mut handshake = IncomingHandshake::new();
        while !handshake.is_finished() {
            let input = connection.read_value()?.value;
            let response = handshake.try_advance(&input.into_array().unwrap()).unwrap();
            connection.write_value(response).unwrap();
        }
        Ok(())
    }
}
