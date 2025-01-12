use std::fmt::Debug;

use crate::connection::{self, stream::Stream};

pub trait RedisListner: Debug {
    type Stream: Stream;
    fn get_port(&self) -> u16;
    fn bind(port: u16) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn incoming(self) -> impl Iterator<Item = Self::Stream>;
}

#[derive(Debug)]
pub struct RedisTcpListner(std::net::TcpListener);

impl RedisListner for RedisTcpListner {
    type Stream = connection::stream::TcpStream;

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

    fn incoming(self) -> impl Iterator<Item = Self::Stream> {
        let binding = Box::leak(Box::new(self.0));
        binding.incoming().map(|stream| stream.unwrap().into())
    }

    fn get_port(&self) -> u16 {
        self.0.local_addr().unwrap().port()
    }
}
