use tracing::instrument;

use crate::{event::EventEmitter, repository::Repository};

use super::{
    handshake::outgoing::OutgoingHandshake,
    stream::{PipelineBuffer, Stream},
};

mod leader_connection;
pub use leader_connection::leader::default_leader_router;

//#[cfg(test)]
//mod tests;

#[allow(clippy::module_name_repetitions)]
pub struct OutgoingConnection<S> {
    connection: PipelineBuffer<S>,
    emitter: EventEmitter,
    repo: Repository,
    router: &'static crate::command::CommandRouter<crate::Request, (), Repository>,
}

impl<S> OutgoingConnection<S>
where
    S: Stream<Addr = std::net::SocketAddrV4>,
{
    pub fn new(
        connection: S,
        router: &'static crate::command::CommandRouter<crate::Request, (), Repository>,
        emitter: EventEmitter,
        repo: Repository,
    ) -> Self {
        Self {
            connection: PipelineBuffer::new(connection),
            router,
            emitter,
            repo,
        }
    }

    pub fn connect(
        addr: std::net::SocketAddrV4,
        router: &'static crate::command::CommandRouter<crate::Request, (), Repository>,
        emitter: EventEmitter,
        repo: Repository,
    ) -> anyhow::Result<Self> {
        Ok(Self::new(S::connect(addr)?, router, emitter, repo))
    }

    #[instrument(name = "outgoing_connection", skip(self))]
    pub fn run(self) -> anyhow::Result<()> {
        leader_connection::LeaderConnection::new(
            self.connection,
            self.router,
            self.emitter,
            self.repo,
        )
        .run()
    }
}
