pub mod config;
pub mod connection;
pub mod event;
pub mod incoming_connection;
pub mod io;
pub mod listner;
pub mod outgoing_connection;
pub mod repository;
pub mod resp;

pub trait Connection: std::io::Read + std::io::Write {}

pub trait RespConnection: Connection {
    fn read_resp(&mut self, buf: &mut [u8]) -> anyhow::Result<usize>;
    fn write_resp(&mut self, buf: &[u8]) -> anyhow::Result<()>;
    fn into_inner(self) -> impl Connection;
}

pub trait RedisCommandsConnection: RespConnection {
    fn read_command(&mut self) -> anyhow::Result<()>;
    fn write_command(&mut self, command: ()) -> anyhow::Result<()>;
    fn into_inner(self) -> impl RespConnection;
}
