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
            let inputs = connection.read_values()?.into_iter().map(|m| m.value);
            let responses = inputs
                .map(|input| handshake.try_advance(&input.into_array().unwrap()).unwrap())
                .collect();
            //let response = handshake.try_advance(&input.into_array().unwrap()).unwrap();
            connection.write_values(responses).unwrap();
        }
        Ok(())
    }
}
