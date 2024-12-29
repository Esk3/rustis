use rustis::{
    config::{RedisConfig, Role},
    incoming_connection::IncomingConnectionHandler,
    listner::RedisListner,
    outgoing_connection::OutgoingConnectionHandler,
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

    fn connect_to_leader<O>(&mut self)
    where
        O: OutgoingConnectionHandler,
    {
        if !self.is_follower() {
            error!("tried to connect to leader without being a follower");
            panic!("is not follower");
        }
        O::connect(
            self.config
                .leader_addr()
                .expect("should be set if follower"),
        );
    }

    fn incoming<I>(self)
    where
        I: IncomingConnectionHandler<Connection = L::Connection>,
    {
        info!("accepting incoming connections");
        for connection in self.listner.incoming() {
            info!("connection accepted");
            I::accept_connection(connection);
        }
    }

    #[instrument(skip(self))]
    pub fn run<I, O>(mut self)
    where
        I: IncomingConnectionHandler<Connection = L::Connection>,
        O: OutgoingConnectionHandler,
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
            self.connect_to_leader::<O>();
            info!("connected to leader");
        }
        self.incoming::<I>();
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
