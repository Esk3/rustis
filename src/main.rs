use std::net::{TcpListener, TcpStream};

use rustis::connection::client::Client;

fn main() {
    let addr = "127.0.0.1:6379";
    let listner = TcpListener::bind(addr).unwrap();
    println!("server listning on: {addr}");
    //for stream in listner.incoming() {
    //    let (tx, rx) = std::sync::mpsc::channel();
    //    let (tx2, rx2) = std::sync::mpsc::channel();
    //    let conn = Connection::new(
    //        rx,
    //        tx2,
    //        rustis::connection::ConnectionType::Client(Client::new()),
    //        stream.unwrap(),
    //    );
    //    conn.run();
    //}
}
