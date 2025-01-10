use std::net::SocketAddr;

use handler::Handler;
use tracing::instrument;

use crate::{connection, repository::Repository};

use super::{handshake::outgoing::OutgoingHandshake, Connection};

mod handler;

//#[cfg(test)]
//mod tests;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct OutgoingConnection<C> {
    connection: C,
    repo: Repository,
}

impl<C> OutgoingConnection<C>
where
    C: Connection,
{
    pub fn new(connection: C, repo: Repository) -> Self {
        Self { connection, repo }
    }

    pub fn connect(addr: SocketAddr, repo: Repository) -> anyhow::Result<Self> {
        Ok(Self::new(C::connect(addr)?, repo))
    }

    fn handshake(&mut self) -> anyhow::Result<usize> {
        let mut handshake = OutgoingHandshake::new();
        let mut response = None;
        while let Some(next) = handshake.try_advance(&response).unwrap() {
            dbg!(&next);
            self.connection.write_value(next).unwrap();
            response = Some(self.connection.read_value()?.value);
        }
        Ok(1)
    }

    #[instrument(skip(self))]
    pub fn run(mut self) -> anyhow::Result<()> {
        tracing::info!("handing connection to leader");
        self.handshake()?;
        tracing::info!("handshake with leader sucesfully completed");
        let mut handler = Handler::new(self.repo);
        loop {
            let message = match self.connection.read_value() {
                Ok(msg) => msg,
                Err(err) => match err {
                    super::ConnectionError::EndOfInput => return Ok(()),
                    super::ConnectionError::Io(_) => todo!(),
                    super::ConnectionError::Any(_) => todo!(),
                },
            };
            tracing::debug!("got message from leader {message:?}");
            let response = handler.handle_request(message.try_into()?)?;
            tracing::debug!("message response: {response:?}");
            if let Some(resposne) = response {
                self.connection.write_value(resposne.into())?;
            }
        }
    }
}
