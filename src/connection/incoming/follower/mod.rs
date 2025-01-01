use anyhow::bail;

use crate::connection::{Connection, ConnectionMessage, Input, Output, ReplConf};

#[cfg(test)]
mod tests;

pub struct Follower<C> {
    connection: C,
}

impl<C> Follower<C>
where
    C: Connection,
{
    pub fn new(connection: C) -> Self {
        Self { connection }
    }

    fn handle_event(
        &mut self,
        event: crate::event::Kind,
    ) -> anyhow::Result<Option<ConnectionMessage>> {
        let res = match event {
            crate::event::Kind::Set { key, value, expiry } => {
                Some(ConnectionMessage::Input(crate::connection::Input::Set {
                    key,
                    value,
                    expiry: None,
                    get: false,
                }))
            }
        };
        Ok(res)
    }

    fn handshake(&mut self) -> anyhow::Result<()> {
        let mut handshake = IncomingHandshake::new();
        while !handshake.is_finished() {
            let response = self
                .connection
                .read_message()
                .unwrap()
                .into_input()
                .unwrap();
            let msg = handshake.get_message().unwrap();
            handshake.handle_message_recived(response).unwrap();
            self.connection.write_message(msg.into()).unwrap();
        }
        Ok(())
    }
}

struct IncomingHandshake {
    input: Vec<Input>,
    output: Vec<Output>,
}
impl IncomingHandshake {
    pub fn new() -> Self {
        Self {
            input: [
                Input::Ping,
                ReplConf::ListingPort(1).into(),
                ReplConf::Capa(String::new()).into(),
                Input::Psync,
            ]
            .to_vec(),
            output: [
                Output::Pong,
                ReplConf::ListingPort(1).into(),
                ReplConf::Capa(String::new()).into(),
                Output::Psync,
            ]
            .to_vec(),
        }
    }

    fn get_all_messages(&self) -> Vec<Input> {
        self.input.clone()
    }

    fn get_all_responses(&self) -> Vec<Output> {
        self.output.clone()
    }

    fn handle_message_recived(&mut self, response: Input) -> anyhow::Result<()> {
        if response != self.input[0] {
            bail!("expected : {:?}", self.input[0]);
        }
        self.advance();
        Ok(())
    }

    fn get_message(&self) -> Option<Output> {
        self.output.first().cloned()
    }

    fn advance(&mut self) {
        self.input.remove(0);
        self.output.remove(0);
    }
    fn is_finished(&self) -> bool {
        self.input.is_empty()
    }
}
