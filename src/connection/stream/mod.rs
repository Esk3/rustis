mod pipeline_buffer;
mod redis_connection;
mod tcpstream;

pub use error::Error;
pub use pipeline_buffer::PipelineBuffer;
pub use redis_connection::RedisConnection;
pub use tcpstream::TcpStream;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;

pub trait Stream: std::io::Read + std::io::Write {
    type Addr;
    fn connect(addr: Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn peer_addr(&self) -> Self::Addr;
}

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("stream closed")]
        StreamClosed,
        #[error("connection closed unexectedly: {0}")]
        ConnectinClosedUnexpectedly(std::io::Error),
        #[error("io error: {0}")]
        IoError(std::io::Error),
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            match value.kind() {
                std::io::ErrorKind::ConnectionRefused
                | std::io::ErrorKind::ConnectionReset
                | std::io::ErrorKind::ConnectionAborted
                | std::io::ErrorKind::NotConnected => Self::ConnectinClosedUnexpectedly(value),
                std::io::ErrorKind::BrokenPipe => Self::StreamClosed,
                std::io::ErrorKind::WouldBlock => todo!(),
                _ => Self::IoError(value),
            }
        }
    }
}
