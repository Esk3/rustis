use crate::{
    config::{RedisConfig, Role},
    connection::{
        incoming::{client, IncomingConnection},
        outgoing::OutgoingConnection,
        stream::Stream,
    },
    event::EventEmitter,
    listner::RedisListner,
    repository::Repository,
};
use tracing::{error, info, instrument};

pub mod builder;

#[cfg(test)]
mod tests;

pub struct Redis<L, C> {
    config: RedisConfig,
    listner: L,
    leader_connection: Option<C>,
    client_router: &'static client::Router,
    repo: Repository,
    emitter: EventEmitter,
}

impl<L, S> Redis<L, S>
where
    L: RedisListner,
    S: Stream<Addr = std::net::SocketAddrV4>,
{
    #[must_use]
    pub fn new(
        listner: L,
        leader_connection: Option<S>,
        repo: Repository,
        emitter: EventEmitter,
    ) -> Self {
        let config = if let Some(ref connection) = leader_connection {
            RedisConfig::new_follower(listner.get_port(), connection.peer_addr().into())
        } else {
            RedisConfig::new(listner.get_port())
        };
        Self {
            config,
            listner,
            leader_connection,
            client_router: client::default_router(),
            repo,
            emitter,
        }
    }

    #[must_use]
    pub fn get_port(&self) -> u16 {
        self.config.port()
    }

    fn connect_to_leader(&mut self) -> anyhow::Result<OutgoingConnection<S>> {
        if !self.is_follower() {
            error!("tried to connect to leader without being a follower");
            panic!("is not follower");
        }
        OutgoingConnection::<S>::connect(
            self.config
                .leader_addr()
                .expect("should be set if follower"),
            self.repo.clone(),
        )
    }

    fn incoming(self)
    where
        <L as RedisListner>::Stream: std::marker::Send + 'static,
    {
        info!("accepting incoming connections");
        for connection in self.listner.incoming() {
            info!("connection accepted");
            let connection = IncomingConnection::new(
                connection,
                self.client_router,
                self.emitter.clone(),
                self.repo.clone(),
            );
            connection.spawn_handler();
        }
    }

    #[instrument(name = "redis_server", skip(self))]
    pub fn run(mut self)
    where
        S: Stream + Send + 'static,
        <L as RedisListner>::Stream: std::marker::Send + 'static,
    {
        info!(
            "starting to run redis server as {}",
            if self.is_leader() {
                "leader"
            } else {
                "follower"
            }
        );
        if self.is_follower() {
            let connection_to_leader = self.connect_to_leader().unwrap();
            info!("connected to leader");
            std::thread::spawn(move || connection_to_leader.run().unwrap());
        }
        self.incoming();
    }

    pub fn spawn(self)
    where
        L: Send + 'static,
        <L as RedisListner>::Stream: Send + 'static,
        S: Send + 'static,
    {
        std::thread::spawn(move || self.run());
    }

    pub fn role(&self) -> Role {
        match self.config.leader_addr() {
            Some(addr) => Role::Follower(addr),
            None => Role::Leader,
        }
    }

    pub fn is_leader(&self) -> bool {
        match self.role() {
            Role::Leader => true,
            Role::Follower(_) => false,
        }
    }

    pub fn is_follower(&self) -> bool {
        !self.is_leader()
    }
}
