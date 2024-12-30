use crate::connection::Connection;

pub trait RedisListner {
    type Connection: Connection;
    fn bind(port: u16) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn incoming(self) -> impl Iterator<Item = Self::Connection>;
}
