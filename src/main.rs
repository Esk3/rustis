use std::net::{Ipv4Addr, SocketAddrV4};

use redis::Redis;
use rustis::{connection::RedisTcpConnection, listner::RedisTcpListner};

pub mod redis;

fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 6379);
    //let listner = std::net::TcpListener::bind(addr).unwrap();
    let redis = Redis::<RedisTcpListner>::bind().unwrap();

    tracing::info!("server listning on: {addr}");

    redis.run::<RedisTcpConnection>();

    tracing::info!("shutting down");
}
