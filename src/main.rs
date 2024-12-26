use std::net::{Ipv4Addr, SocketAddrV4};

use rustis::{
    connection::connection_handler,
    event::LockEventProducer,
    repository::LockingMemoryRepository,
    resp::parser::{RespEncoder, RespParser},
};

fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 6379);
    let listner = std::net::TcpListener::bind(addr).unwrap();

    tracing::info!("server listning on: {addr}");

    let repo = LockingMemoryRepository::new();
    let event = LockEventProducer::new();

    for stream in listner.incoming() {
        tracing::info!("handling new connection");
        let stream = stream.unwrap();
        let stream = rustis::io::TcpStream::new(&stream);
        connection_handler::<RespParser, RespEncoder, _, _, _>(stream, event.clone(), repo.clone());
    }
    tracing::info!("shutting down");
}
