use rustis::{
    config::{RedisConfig, Role},
    connection::{incoming::IncomingConnection, outgoing::OutgoingConnection, Connection},
    event::EventEmitter,
    listner::RedisListner,
    repository::LockingMemoryRepository,
};
use tracing::{error, info, instrument};

#[cfg(test)]
mod tests;

pub struct Redis<L> {
    config: RedisConfig,
    listner: L,
    repo: LockingMemoryRepository,
    emitter: EventEmitter,
}

impl<L> Redis<L>
where
    L: RedisListner,
{
    pub fn bind() -> anyhow::Result<Self> {
        let config = RedisConfig::new(6379);
        Self::bind_from_config(config)
    }

    pub fn bind_from_config(config: RedisConfig) -> anyhow::Result<Self> {
        let listner = L::bind(config.port())?;
        Ok(Self {
            config,
            listner,
            repo: LockingMemoryRepository::new(),
            emitter: EventEmitter::new(),
        })
    }

    #[must_use]
    pub fn get_port(&self) -> u16 {
        self.config.port()
    }

    fn connect_to_leader<C>(&mut self)
    where
        C: Connection,
    {
        if !self.is_follower() {
            error!("tried to connect to leader without being a follower");
            panic!("is not follower");
        }
        OutgoingConnection::<C>::connect(
            self.config
                .leader_addr()
                .expect("should be set if follower"),
        );
    }

    fn incoming(self) {
        info!("accepting incoming connections");
        for connection in self.listner.incoming() {
            info!("connection accepted");
            let connection =
                IncomingConnection::new(connection, self.emitter.clone(), self.repo.clone());
            connection.handle_connection().unwrap();
        }
    }

    #[instrument(skip(self))]
    pub fn run<C>(mut self)
    where
        C: Connection,
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
            self.connect_to_leader::<C>();
            info!("connected to leader");
        }
        self.incoming();
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
