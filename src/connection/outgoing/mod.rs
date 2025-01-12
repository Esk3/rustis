use std::net::SocketAddr;

use tracing::instrument;

use crate::{connection, repository::Repository, resp};

use super::{
    handshake::outgoing::OutgoingHandshake,
    stream::{PipelineBuffer, Stream},
};

mod handler;

//#[cfg(test)]
//mod tests;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct OutgoingConnection<S> {
    connection: PipelineBuffer<S>,
    repo: Repository,
}

impl<S> OutgoingConnection<S>
where
    S: Stream<Addr = std::net::SocketAddrV4>,
{
    pub fn new(connection: S, repo: Repository) -> Self {
        Self {
            connection: PipelineBuffer::new(connection),
            repo,
        }
    }

    pub fn connect(addr: std::net::SocketAddrV4, repo: Repository) -> anyhow::Result<Self> {
        Ok(Self::new(S::connect(addr)?, repo))
    }

    #[instrument(name = "outgoing_connection", skip(self))]
    pub fn run(self) -> anyhow::Result<()> {
        LeaderConnection::new(self.connection).run()
    }
}

struct LeaderConnection<S> {
    connection: PipelineBuffer<S>,
    leader: Leader,
}

impl<S> LeaderConnection<S>
where
    S: Stream,
{
    fn new(connection: PipelineBuffer<S>) -> Self {
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
            let message = match self.connection.read() {
                Ok(msg) => msg,
                Err(err) => todo!("{err}"),
                //Err(err) => match err {
                //    super::ConnectionError::EndOfInput => return Ok(()),
                //    super::ConnectionError::Io(_) => todo!(),
                //    super::ConnectionError::Any(_) => todo!(),
                //},
            };
            tracing::debug!("got message from leader {message:?}");
            let response = self.leader.handle_request(()).unwrap();
            //    if responses.is_empty() {
            //        tracing::debug!("no response");
            //    } else {
            //        tracing::debug!("message response: {responses:?}");
            //        self.connection.write_values(responses).unwrap();
            //    }
            todo!()
        }
    }

    fn handshake(&mut self) -> anyhow::Result<usize> {
        let mut handshake = OutgoingHandshake::new();
        let mut response = None;
        while let Some(next) = handshake.try_advance(&response).unwrap() {
            dbg!(&next);
            self.connection.write(&next).unwrap();
            let value = self.connection.read().unwrap().value;
            let arr = value.into_array().unwrap_or_else(|v| vec![v]);
            response = Some(arr);
        }
        todo!("read rdb file");
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
