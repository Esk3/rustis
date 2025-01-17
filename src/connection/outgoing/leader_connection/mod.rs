use leader::Leader;

use crate::{
    connection::{
        outgoing::OutgoingHandshake,
        stream::{PipelineBuffer, Stream},
    },
    event::{EmitAll, EventEmitter},
    repository::Repository,
};

pub mod leader;

pub struct LeaderConnection<S> {
    connection: PipelineBuffer<S>,
    leader: Leader,
    emitter: EventEmitter,
}

impl<S> LeaderConnection<S>
where
    S: Stream,
{
    pub fn new(
        connection: PipelineBuffer<S>,
        router: &'static crate::command::CommandRouter<crate::Request, (), Repository>,
        emitter: EventEmitter,
        repo: Repository,
    ) -> Self {
        Self {
            connection,
            leader: Leader::new(router, repo),
            emitter,
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        tracing::info!("handing connection to leader");
        self.handshake()?;
        tracing::info!("handshake with leader sucesfully completed");
        loop {
            let message = match self.connection.read() {
                Ok(msg) => msg,
                Err(err) => todo!("{err}"),
            };
            tracing::debug!("got message from leader {message:?}");
            let leader::LeaderResponse { value, events } =
                self.leader.handle_request(message.into()).unwrap();

            if let Some(response) = value {
                tracing::debug!("sending value [{response:?}]");
                self.connection.write(&response).unwrap();
            }

            if let Some(events) = events {
                events.emit_all(&self.emitter);
            }
        }
    }

    fn handshake(&mut self) -> anyhow::Result<usize> {
        let mut handshake = OutgoingHandshake::new();
        let mut response = None;
        while let Some(next) = handshake.try_advance(&response).unwrap() {
            self.connection.write(&next.into()).unwrap();
            let message = self.connection.read().unwrap();
            response = Some(message);
        }
        let mut rdb_buf = [0; 1024];
        std::thread::sleep(std::time::Duration::from_secs(1));
        let bytes_read = self.connection.inner().inner().read(&mut rdb_buf).unwrap();
        assert_ne!(bytes_read, rdb_buf.len(), "rdb buffer overflow");
        tracing::debug!("{:?}", &rdb_buf[..bytes_read]);
        tracing::debug!("{bytes_read}");
        Ok(1)
    }
}
