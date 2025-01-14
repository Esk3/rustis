mod pipeline_buffer;
mod redis_connection;
mod tcpstream;

pub use pipeline_buffer::PipelineBuffer;
pub use redis_connection::RedisConnection;
pub use tcpstream::TcpStream;

#[cfg(test)]
mod tests;

pub trait Stream: std::io::Read + std::io::Write {
    type Addr;
    fn connect(addr: Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn peer_addr(&self) -> Self::Addr;
}
