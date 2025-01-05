use std::net::SocketAddr;

use tracing::instrument;

use super::{handshake::outgoing::OutgoingHandshake, Connection};
use crate::resp::{Input, Message, Output, ReplConf};

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
        let mut response = None;
        while !handshake.is_finished() {
            let message = handshake.try_advance(&response).unwrap();
            self.connection.write_message(message.into()).unwrap();
            response = Some(self.connection.read_message()?.into_output().unwrap());
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
                    super::ConnectionError::Any(_) => todo!(),
                },
            };
        }
    }
}
