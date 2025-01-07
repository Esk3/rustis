use std::fmt::Debug;

use crate::connection::{connections::RedisTcpConnection, Connection};

pub trait RedisListner: Debug {
    type Connection: Connection;
    fn get_port(&self) -> u16;
    fn bind(port: u16) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn incoming(self) -> impl Iterator<Item = Self::Connection>;
}

#[derive(Debug)]
pub struct RedisTcpListner(std::net::TcpListener);

impl RedisListner for RedisTcpListner {
    type Connection = RedisTcpConnection;

    fn bind(port: u16) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let listner = std::net::TcpListener::bind(std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::LOCALHOST,
            port,
        ));
        Ok(Self(listner?))
    }

    fn incoming(self) -> impl Iterator<Item = Self::Connection> {
        let binding = Box::leak(Box::new(self.0));
        binding.incoming().map(|stream| stream.unwrap().into())
    }

    fn get_port(&self) -> u16 {
        self.0.local_addr().unwrap().port()
    }
}
