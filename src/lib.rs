use thiserror::Error;

pub mod config;
pub mod connection;
pub mod event;
pub mod io;
pub mod listner;
pub mod repository;
pub mod resp;

pub trait Connection {
    fn read_resp(&mut self, buf: &mut [u8]) -> ConnectionResult<usize>;
    fn write_resp(&mut self, buf: &[u8]) -> ConnectionResult<()>;
    fn from_connection<C>(value: C) -> Self;
    fn read_command(&mut self) -> ConnectionResult<()>;
    fn write_command(&mut self, command: ()) -> ConnectionResult<()>;
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("end of input")]
    EndOfInput,
}

pub type ConnectionResult<T> = Result<T, ConnectionError>;
