use std::net::{Ipv4Addr, SocketAddrV4};

use rustis::redis::builder::RedisBuilder;
use rustis::{
    connection::connections::RedisTcpConnection,
    event::EventEmitter,
    listner::{RedisListner, RedisTcpListner},
    repository::Repository,
};

fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 6379);
    let repo = Repository::default();
    let emitter = EventEmitter::new();

    let redis = RedisBuilder::<RedisTcpListner, RedisTcpConnection>::new()
        .listner(RedisTcpListner::bind(6379).unwrap())
        .repo(repo)
        .emitter(emitter)
        .build()
        .unwrap();

    tracing::info!("server listning on: {addr}");

    redis.run();

    tracing::info!("shutting down");
}
