use tracing::instrument;

use crate::{event::EventEmitter, repository::Repository};

use super::{
    handshake::outgoing::OutgoingHandshake,
    stream::{PipelineBuffer, Stream},
};

mod handler;
mod leader_connection;

//#[cfg(test)]
//mod tests;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct OutgoingConnection<S> {
    connection: PipelineBuffer<S>,
    emitter: EventEmitter,
    repo: Repository,
}

impl<S> OutgoingConnection<S>
where
    S: Stream<Addr = std::net::SocketAddrV4>,
{
    pub fn new(connection: S, emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            connection: PipelineBuffer::new(connection),
            emitter,
            repo,
        }
    }

    pub fn connect(
        addr: std::net::SocketAddrV4,
        emitter: EventEmitter,
        repo: Repository,
    ) -> anyhow::Result<Self> {
        Ok(Self::new(S::connect(addr)?, emitter, repo))
    }

    #[instrument(name = "outgoing_connection", skip(self))]
    pub fn run(self) -> anyhow::Result<()> {
        leader_connection::LeaderConnection::new(self.connection, self.emitter, self.repo).run()
    }
}
