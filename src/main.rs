use std::net::{Ipv4Addr, SocketAddrV4};

use redis::Redis;
use rustis::{
    event::LockEventProducer,
    repository::LockingMemoryRepository,
    resp::parser::{RespEncoder, RespParser},
};

pub mod redis;

fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 6379);
    let listner = std::net::TcpListener::bind(addr).unwrap();

    tracing::info!("server listning on: {addr}");

    let repo = LockingMemoryRepository::new();
    let event = LockEventProducer::new();

    tracing::info!("shutting down");
}
