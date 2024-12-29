use crate::{Connection, RedisCommandsConnection, RespConnection};

#[cfg(test)]
mod tests;

pub trait IncomingConnectionHandler {
    type Connection: Connection;
    fn accept_connection(connection: Self::Connection);
}

pub struct IncomingRedisConnectionHandler<C> {
    pd: std::marker::PhantomData<C>,
}

impl<C> IncomingRedisConnectionHandler<C>
where
    C: RedisCommandsConnection,
{
    fn new(connection: C) -> Self {
        Self {
            pd: std::marker::PhantomData,
        }
    }
}

impl IncomingConnectionHandler for IncomingRedisConnectionHandler<()> {
    type Connection = RedisTcpConnection;
    fn accept_connection(connection: Self::Connection) {
        todo!()
    }
}

pub struct RedisTcpConnection;

impl RespConnection for RedisTcpConnection {
    fn read_resp(&mut self, buf: &mut [u8]) -> anyhow::Result<usize> {
        todo!()
    }

    fn write_resp(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        todo!()
    }

    fn into_inner(self) -> impl Connection {
        self
    }
}

impl Connection for RedisTcpConnection {}

impl std::io::Read for RedisTcpConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
impl std::io::Write for RedisTcpConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
