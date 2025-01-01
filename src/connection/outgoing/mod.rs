use std::net::SocketAddr;

use tracing::instrument;

use super::{Connection, ConnectionMessage, Input, Output, ReplConf};

mod handler;

#[cfg(test)]
mod tests;

pub struct OutgoingConnection<C> {
    connection: C,
}

impl<C> OutgoingConnection<C>
where
    C: Connection,
{
    pub fn new(connection: C) -> Self {
        Self { connection }
    }

    pub fn connect(addr: SocketAddr) -> anyhow::Result<Self> {
        Ok(Self {
            connection: C::connect(addr)?,
        })
    }

    fn handshake(&mut self) -> anyhow::Result<usize> {
        let mut handshake = OutgoingHandshake::new();
        while !handshake.is_finished() {
            let message = handshake.get_message();
            self.connection.write_message(message.into()).unwrap();
            let response = self.connection.read_message()?;
            handshake.handle_response(response).unwrap();
        }
        Ok(1)
    }

    #[instrument(skip(self))]
    fn run(mut self) -> anyhow::Result<()> {
        self.handshake()?;
        loop {
            let message = match self.connection.read_message() {
                Ok(msg) => msg,
                Err(err) => match err {
                    super::ConnectionError::EndOfInput => return Ok(()),
                },
            };
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OutgoingHandshake {
    i: usize,
    requests: [Input; 4],
    responses: [Output; 4],
}

impl OutgoingHandshake {
    fn new() -> Self {
        Self {
            i: 0,
            requests: [
                Input::Ping,
                Input::ReplConf(ReplConf::ListingPort(1)),
                Input::ReplConf(ReplConf::Capa(String::new())),
                Input::Psync,
            ],
            responses: [
                Output::Pong,
                ReplConf::ListingPort(1).into(),
                ReplConf::Capa(String::new()).into(),
                Output::Psync,
            ],
        }
    }

    fn next(&mut self) {
        self.i += 1;
    }

    pub fn get_message(&self) -> Input {
        self.requests[self.i].clone()
    }

    fn expected_message(&self) -> Output {
        self.responses[self.i].clone()
    }

    pub fn handle_response(&mut self, message: ConnectionMessage) -> anyhow::Result<()> {
        let response = message.into_output().unwrap();
        let expected = self.expected_message();
        assert_eq!(response, expected);
        self.next();
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.i == self.requests.len()
    }

    fn get_all_messages(mut self) -> Vec<Input> {
        self.requests[self.i..].to_vec()
    }
}
