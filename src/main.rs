use std::net::{Ipv4Addr, SocketAddrV4};

use rustis::redis::builder::RedisBuilder;
use rustis::{
    connection::connections::{stdio::RedisStdInOutConnection, RedisTcpConnection},
    event::EventEmitter,
    listner::{RedisListner, RedisTcpListner},
    repository::Repository,
};

fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 6379);
    let repo = Repository::default();
    let emitter = EventEmitter::new();
    let std_redis = RedisBuilder::<RedisStdInOutConnection, RedisTcpConnection>::new()
        .listner(RedisStdInOutConnection::new())
        .repo(repo.clone())
        .emitter(emitter.clone())
        .build()
        .unwrap();

    let redis = RedisBuilder::<RedisTcpListner, RedisTcpConnection>::new()
        .listner(RedisTcpListner::bind(6379).unwrap())
        .repo(repo)
        .emitter(emitter)
        .build()
        .unwrap();

    tracing::info!("server listning on: {addr}");

    std_redis.spawn();
    redis.run();

    tracing::info!("shutting down");
}
