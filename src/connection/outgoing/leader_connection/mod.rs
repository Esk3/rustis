use leader::Leader;
use tracing::instrument;

use crate::{
    connection::{
        outgoing::OutgoingHandshake,
        stream::{PipelineBuffer, Stream},
    },
    event::{EmitAll, EventEmitter},
    repository::Repository,
    resp::value::deserialize::util::GetHeader,
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
        self.handle_requests();
        Ok(())
    }

    fn handle_requests(mut self) {
        let mut request_id = 0;
        loop {
            request_id += 1;
            self.handle_request(request_id).unwrap();
        }
    }

    #[instrument(name = "handle_leader_request", skip(self))]
    fn handle_request(&mut self, request_id: usize) -> Result<(), error::Error> {
        let message = self.connection.read()?;

        tracing::debug!("got message from leader {message:?}");
        let leader::LeaderResponse { value, events } =
            self.leader.handle_request(message.into()).unwrap();

        if let Some(response) = value {
            tracing::debug!("sending value [{response:?}]");
            self.connection.write(&response)?;
        }

        if let Some(events) = events {
            events.emit_all(&self.emitter);
        }
        Ok(())
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
        let mut bytes_read = 0;
        let max_reads = 30;
        while bytes_read < max_reads && !rdb_buf.contains(&b'\n') {
            if bytes_read > 0 {
                assert_eq!(
                    rdb_buf[0], b'$',
                    "expected rdb file header got: {}",
                    rdb_buf[0] as char
                );
            }
            bytes_read += self
                .connection
                .inner()
                .inner()
                .read(&mut rdb_buf[bytes_read..])
                .unwrap();
        }
        let (file_size, header_size) = rdb_buf.get_header().unwrap();
        let read_of_file = bytes_read - header_size;
        let file_bytes_remaining = TryInto::<usize>::try_into(file_size).unwrap() - read_of_file;
        assert!(file_bytes_remaining < 1024, "rdb file too large");
        self.connection
            .inner()
            .inner()
            .read_exact(&mut rdb_buf[..file_bytes_remaining])
            .unwrap();
        //tracing::debug!("{:?}", &rdb_buf[..bytes_read]);
        //tracing::debug!("{bytes_read}");
        Ok(1)
    }
}

pub mod error {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("stream error: {0}")]
        Stream(#[from] crate::connection::stream::Error),
    }
}
