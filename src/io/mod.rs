use std::usize;

use crate::resp::Value;

#[derive(Debug)]
pub struct Io<R, W, E, P> {
    r: R,
    w: W,
    encoder: E,
    parser: P,
}

impl<R, W, E, P> Io<R, W, E, P>
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    pub fn new(r: R, w: W, encoder: E, parser: P) -> Self {
        Self {
            r,
            w,
            encoder,
            parser,
        }
    }
}

impl<R, W, E, P> Io<R, W, E, P>
where
    R: std::io::Read,
    E: Encoder,
    P: Parser,
{
    pub fn read_input(&mut self) -> anyhow::Result<Input> {
        let (value, bytes_read) = self.encoder.decode(&mut self.r)?;
        self.parser.parse(value)
    }
}

impl<R, W, E, P> Io<R, W, E, P>
where
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    pub fn write_output(&mut self, output: Output) -> anyhow::Result<usize> {
        let value = self.parser.into_value(output)?;
        self.encoder.encode_to(value, &mut self.w)
    }
    pub fn write_input(&mut self, input: Input) -> anyhow::Result<usize> {
        let value = self.parser.input_into_value(input)?;
        self.encoder.encode_to(value, &mut self.w)
    }
}
impl<R, W, E, P> Io<R, W, E, P>
where
    W: std::io::Write,
{
    pub fn write_raw(&mut self, buf: &[u8]) -> anyhow::Result<usize> {
        self.w.write_all(buf)?;
        Ok(buf.len())
    }
}

impl<R, W, E, P> Io<R, W, E, P> {
    pub fn encoder(&self) -> &E {
        &self.encoder
    }

    pub fn encoder_mut(&mut self) -> &mut E {
        &mut self.encoder
    }

    pub fn parser(&self) -> &P {
        &self.parser
    }

    pub fn parser_mut(&mut self) -> &mut P {
        &mut self.parser
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
}

pub trait Encoder {
    fn encode_to<W>(&mut self, value: Value, w: W) -> anyhow::Result<usize>
    where
        W: std::io::Write;
    fn encode(&mut self, value: Value) -> anyhow::Result<(Vec<u8>)> {
        let mut buf = Vec::new();
        self.encode_to(value, &mut buf)?;
        Ok(buf)
    }
    fn decode<R>(&mut self, r: R) -> anyhow::Result<(Value, usize)>
    where
        R: std::io::Read;
}
pub trait Parser {
    fn parse(&mut self, value: Value) -> anyhow::Result<Input>;
    fn into_value(&mut self, output: Output) -> anyhow::Result<Value>;
    fn input_into_value(&mut self, input: Input) -> anyhow::Result<Value>;
}
