use std::net::TcpListener;

use rustis::{
    api::Api,
    connection::{client::Client, Connection},
    node::Node,
    node_service::node_worker,
    repository::Repository,
};

fn main() {
    let addr = "127.0.0.1:6379";
    let listner = TcpListener::bind(addr).unwrap();
    println!("server listning on: {addr}");

    let service = node_worker::run(Node, Repository::new());

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        let api = Api::from_tcp_stream(&stream);
        let conn = Connection::new(
            rustis::connection::wrapper::ConnectionKind::Client(Client::new(service.clone())),
            api,
        );
        conn.run();
    }
}
