use leader::Leader;

use crate::{
    connection::{
        outgoing::OutgoingHandshake,
        stream::{PipelineBuffer, Stream},
    },
    event::EventEmitter,
    repository::Repository,
};

mod leader;

pub struct LeaderConnection<S> {
    connection: PipelineBuffer<S>,
    leader: Leader,
}

impl<S> LeaderConnection<S>
where
    S: Stream,
{
    pub fn new(connection: PipelineBuffer<S>, emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            connection,
            leader: Leader::new((), emitter, repo),
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
                //Err(err) => match err {
                //    super::ConnectionError::EndOfInput => return Ok(()),
                //    super::ConnectionError::Io(_) => todo!(),
                //    super::ConnectionError::Any(_) => todo!(),
                //},
            };
            tracing::debug!("got message from leader {message:?}");
            let request = todo!();
            let response = self.leader.handle_request(request).unwrap();
            tracing::debug!("sending response {response:?}");
            if let Some(response) = response {
                self.connection.write(&response).unwrap();
            }
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
        let mut rdb_buf = [0; 1024];
        let bytes_read = self.connection.inner().inner().read(&mut rdb_buf).unwrap();
        assert_ne!(bytes_read, rdb_buf.len(), "rdb buffer overflow");
        Ok(1)
    }
}
