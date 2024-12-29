use std::net::SocketAddr;

pub trait OutgoingConnectionHandler {
    fn connect(addr: SocketAddr)
    where
        Self: Sized;
}
