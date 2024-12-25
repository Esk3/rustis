use crate::resp::Value;

pub trait Io {
    fn read_value(&mut self) -> anyhow::Result<Value>;
    fn write_value(&mut self, value: Value) -> anyhow::Result<usize>;
}

pub struct TcpStream {}
impl Io for TcpStream {
    fn read_value(&mut self) -> anyhow::Result<Value> {
        todo!()
    }

    fn write_value(&mut self, value: Value) -> anyhow::Result<usize> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkMessage {
    Input(Input),
    Output(Output),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Ping,

    Get(String),
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
        get: bool,
    },

    ReplConf,
    Psync,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Pong,
    Get(Option<String>),
    Set,

    ReplConf,
    Psync,
    Null,
    Ok,
    Array(Vec<Self>),
}

#[cfg(test)]
pub mod tests {
    use super::Io;

    pub struct Mock {}
    impl Io for Mock {
        fn read_value(&mut self) -> anyhow::Result<crate::resp::Value> {
            todo!()
        }

        fn write_value(&mut self, value: crate::resp::Value) -> anyhow::Result<usize> {
            todo!()
        }
    }
}
