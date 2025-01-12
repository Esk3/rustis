use std::net::SocketAddrV4;

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Leader,
    Follower(SocketAddrV4),
}

#[derive(Debug)]
pub struct RedisConfig {
    port: u16,
    leader_addr: Option<SocketAddrV4>,
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
    pub fn new_follower(port: u16, addr: SocketAddrV4) -> RedisConfig {
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
    pub fn leader_addr(&self) -> Option<SocketAddrV4> {
        self.leader_addr
    }
}
