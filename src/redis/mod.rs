use rustis::{
    config::{RedisConfig, Role},
    connection::{incoming::IncomingConnection, outgoing::OutgoingConnection},
    listner::RedisListner,
};
use tracing::{error, info, instrument};

#[cfg(test)]
mod tests;

pub struct Redis<L> {
    config: RedisConfig,
    listner: L,
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
        Ok(Self { config, listner })
    }

    #[must_use]
    pub fn get_port(&self) -> u16 {
        self.config.port()
    }

    fn connect_to_leader(&mut self) {
        if !self.is_follower() {
            error!("tried to connect to leader without being a follower");
            panic!("is not follower");
        }
        OutgoingConnection::connect(
            self.config
                .leader_addr()
                .expect("should be set if follower"),
        );
    }

    fn incoming(self) {
        info!("accepting incoming connections");
        for connection in self.listner.incoming() {
            info!("connection accepted");
            let connection = IncomingConnection::new(connection);
            connection.handle_connection();
        }
    }

    #[instrument(skip(self))]
    pub fn run(mut self) {
        info!(
            "starting to run redis server as {}",
            if self.is_leader() {
                "leader"
            } else {
                "follower"
            }
        );
        if self.is_follower() {
            self.connect_to_leader();
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
