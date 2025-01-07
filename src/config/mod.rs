use std::net::SocketAddr;

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Leader,
    Follower(SocketAddr),
}

#[derive(Debug)]
pub struct RedisConfig {
    port: u16,
    leader_addr: Option<SocketAddr>,
}

impl RedisConfig {
    #[must_use]
    pub fn new(port: u16) -> Self {
        Self {
            port,
            leader_addr: None,
        }
    }

    #[must_use]
    pub fn new_follower(port: u16, addr: SocketAddr) -> RedisConfig {
        Self {
            port,
            leader_addr: Some(addr),
        }
    }

    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }

    #[must_use]
    pub fn leader_addr(&self) -> Option<SocketAddr> {
        self.leader_addr
    }
}
