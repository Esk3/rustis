use std::net::{Ipv4Addr, SocketAddrV4};

use clap::Parser;
use rustis::connection::stream::{self, Stream};
use rustis::redis::builder::RedisBuilder;
use rustis::{
    event::EventEmitter,
    listner::{RedisListner, RedisTcpListner},
    repository::Repository,
};

fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let port = args.port.unwrap_or(6379);
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let repo = Repository::default();
    let emitter = EventEmitter::new();

    let builder = RedisBuilder::<RedisTcpListner, stream::TcpStream>::new()
        .listner(RedisTcpListner::bind(port).unwrap())
        .repo(repo)
        .emitter(emitter);

    let redis = if let Some(leader_port) = args.replicaof {
        builder.leader_connection(
            stream::TcpStream::connect(SocketAddrV4::new(Ipv4Addr::LOCALHOST, leader_port))
                .unwrap(),
        )
    } else {
        builder
    }
    .build()
    .unwrap();

    tracing::info!("server listning on: {addr}");

    redis.run();

    tracing::info!("shutting down");
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    port: Option<u16>,

    #[arg(long)]
    replicaof: Option<u16>,
}
