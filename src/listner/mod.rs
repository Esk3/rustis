use crate::connection::{Connection, RedisTcpConnection};

pub trait RedisListner {
    type Connection: Connection;
    fn bind(port: u16) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn incoming(self) -> impl Iterator<Item = Self::Connection>;
}

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
}
