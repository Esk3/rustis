use std::net::SocketAddr;

use tracing::instrument;

use crate::{connection, repository::Repository, resp};

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

    #[instrument(name = "outgoing_connection", skip(self))]
    pub fn run(self) -> anyhow::Result<()> {
        LeaderConnection::new(self.connection).run()
    }
}

struct LeaderConnection<C> {
    connection: C,
    leader: Leader,
}

impl<C> LeaderConnection<C>
where
    C: Connection,
{
    fn new(connection: C) -> Self {
        Self {
            connection,
            leader: Leader::new(()),
        }
    }

    fn run(mut self) -> anyhow::Result<()> {
        tracing::info!("handing connection to leader");
        self.handshake()?;
        tracing::info!("handshake with leader sucesfully completed");
        loop {
            let message = match self.connection.read_values() {
                Ok(msg) => msg,
                Err(err) => match err {
                    super::ConnectionError::EndOfInput => return Ok(()),
                    super::ConnectionError::Io(_) => todo!(),
                    super::ConnectionError::Any(_) => todo!(),
                },
            };
            tracing::debug!("got message from leader {message:?}");
            let responses = message
                .into_iter()
                .filter_map(|req| self.leader.handle_request(()).unwrap())
                .collect::<Vec<_>>();
            if responses.is_empty() {
                tracing::debug!("no response");
            } else {
                tracing::debug!("message response: {responses:?}");
                self.connection.write_values(responses).unwrap();
            }
        }
    }

    fn handshake(&mut self) -> anyhow::Result<usize> {
        let mut handshake = OutgoingHandshake::new();
        let mut input = Vec::new();
        let mut response = None;
        while let Some(next) = handshake.try_advance(&response).unwrap() {
            dbg!(&next);
            self.connection.write_values(vec![next]).unwrap();
            if input.is_empty() {
                input.extend(self.connection.read_values().unwrap());
            }
            let value = input.remove(0).value;
            let arr = value.into_array().unwrap_or_else(|v| vec![v]);
            response = Some(arr);
        }
        if input.is_empty() {
            let rdb = self.connection.read_values().unwrap();
        } else {
            let rdb = input.remove(0);
        }
        Ok(1)
    }
}

struct Leader {
    router: (),
}

impl Leader {
    fn new(router: ()) -> Self {
        Self { router }
    }

    fn handle_request(&mut self, request: ()) -> anyhow::Result<Option<resp::Value>> {
        todo!()
    }
}
