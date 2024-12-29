use std::io::{BufReader, BufWriter};

use thiserror::Error;

use crate::resp::Value;

pub trait Io {
    fn read_value(&mut self) -> IoResult<Value>;
    fn write_value(&mut self, value: Value) -> IoResult<usize>;
}

pub trait ValueIo {}
pub trait MessageIo {}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("end of input")]
    EndOfInput,
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
}

pub type IoResult<T> = Result<T, IoError>;

#[derive(Debug)]
pub struct TcpStream<'a> {
    r: BufReader<&'a std::net::TcpStream>,
    w: BufWriter<&'a std::net::TcpStream>,
}

impl<'a> TcpStream<'a> {
    #[must_use]
    pub fn new(stream: &'a std::net::TcpStream) -> Self {
        Self {
            r: BufReader::new(stream),
            w: BufWriter::new(stream),
        }
    }
}
impl Io for TcpStream<'_> {
    fn read_value(&mut self) -> IoResult<Value> {
        todo!("read value from stream")
    }

    fn write_value(&mut self, value: Value) -> IoResult<usize> {
        todo!("write {value:?} to stream")
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

    Multi,
    CommitMulti,

    ReplConf(ReplConf),
    Psync,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Pong,
    Get(Option<String>),
    Set,

    Multi,
    Queued,

    ReplConf(ReplConf),
    Psync,
    Null,
    Ok,
    Array(Vec<Self>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReplConf {
    ListingPort(u16),
    Capa(String),
    GetAck(i32),
    Ack(i32),
}

#[cfg(test)]
pub mod tests {
    use crate::resp::Value;

    use super::{Io, IoResult};

    #[derive(Debug)]
    pub struct MockIo {
        inputs: Vec<Value>,
        outputs: Vec<Value>,
    }

    impl MockIo {
        pub fn new<I, O>(inputs: I, outputs: O) -> Self
        where
            I: IntoIterator<Item = Value>,
            <I as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
            O: IntoIterator<Item = Value>,
            <O as std::iter::IntoIterator>::IntoIter: std::iter::DoubleEndedIterator,
        {
            Self {
                inputs: inputs.into_iter().rev().collect(),
                outputs: outputs.into_iter().rev().collect(),
            }
        }
    }
    impl Io for MockIo {
        fn read_value(&mut self) -> IoResult<crate::resp::Value> {
            let value = self.inputs.pop().expect("got more reads than expected");
            if value.eq_ignore_ascii_case("end") {
                return Err(super::IoError::EndOfInput);
            }
            Ok(value)
        }

        fn write_value(&mut self, value: crate::resp::Value) -> IoResult<usize> {
            let expected = self.outputs.pop().expect("got more writes than expected");
            assert_eq!(value, expected, "writes don't match");
            Ok(1)
        }
    }

    impl Drop for MockIo {
        fn drop(&mut self) {
            if std::thread::panicking() {
                return;
            }
            assert!(
                self.inputs.is_empty() || self.outputs.is_empty(),
                "got less reads and writes than expected: {self:?}"
            );
            assert!(
                self.inputs.is_empty(),
                "got less reads than expected: {:?}",
                self.inputs
            );
            assert!(
                self.outputs.is_empty(),
                "got less writes than expected: {:?}",
                self.outputs
            );
        }
    }

    #[test]
    #[should_panic(expected = "got less reads than expected")]
    fn mock_panics_on_too_few_reads() {
        _ = MockIo::new([Value::SimpleString("_".into())], []);
    }
    #[test]
    #[should_panic(expected = "got more reads than expected")]
    fn mock_panics_on_too_many_reads() {
        let mut io = MockIo::new([], []);
        _ = io.read_value().unwrap();
    }
    #[test]
    fn mock_produces_expected_reads() {
        let inputs = [
            Value::SimpleString("a".into()),
            Value::BulkString("b".into()),
        ];
        let mut io = MockIo::new(inputs.clone(), []);
        let res = [io.read_value().unwrap(), io.read_value().unwrap()];
        assert_eq!(res, inputs);
    }
    #[test]
    #[should_panic(
        expected = "assertion `left == right` failed: writes don't match\n  left: SimpleString(\"wrong\")\n right: SimpleString(\"exp\")"
    )]
    fn mock_panics_on_write_not_matcing() {
        let mut io = MockIo::new([], [Value::SimpleString("exp".into())]);
        io.write_value(Value::SimpleString("wrong".into())).unwrap();
    }

    #[test]
    #[should_panic(expected = "got less writes than expected")]
    fn mock_panics_on_too_few_writes() {
        _ = MockIo::new([], [Value::SimpleString("_".into())]);
    }

    #[test]
    #[should_panic(expected = "got more writes than expected")]
    fn mock_panics_on_too_many_writes() {
        let mut io = MockIo::new([], []);
        io.write_value(Value::SimpleString("_".into())).unwrap();
    }

    #[test]
    #[should_panic(expected = "got less reads and writes than expected")]
    fn mock_panics_on_too_few_reads_and_writes() {
        _ = MockIo::new(
            [Value::SimpleString("_".into())],
            [Value::SimpleString("_".into())],
        );
    }
}
