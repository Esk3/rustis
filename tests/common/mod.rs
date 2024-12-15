use anyhow::bail;
use rustis::{
    connection::CallResult,
    io::{Encoder, Input, NetworkMessage, Output, Parser},
    resp::Value,
    service::Service,
};

pub fn test_timeout_millis<F, T>(millis: u64, f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::spawn(f);
    std::thread::sleep(std::time::Duration::from_millis(millis));
    assert!(handle.is_finished(), "test timed out");
    handle.join().unwrap()
}

pub fn test_timeout<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    test_timeout_millis(200, f)
}

#[derive(Debug)]
pub struct MockEncoder;

impl Encoder for MockEncoder {
    fn encode_to<W>(&mut self, value: rustis::resp::Value, mut w: W) -> anyhow::Result<usize>
    where
        W: std::io::Write,
    {
        Ok(1)
    }

    fn decode<R>(&mut self, r: R) -> anyhow::Result<(rustis::resp::Value, usize)>
    where
        R: std::io::Read,
    {
        Ok((Value::SimpleString("UNIMPLIMENTED".into()), 1))
    }
}

#[derive(Debug)]
pub struct MockParser {
    inputs: Vec<NetworkMessage>,
    ouputs: Vec<NetworkMessage>,
    expected: Option<Vec<NetworkMessage>>,
}
impl MockParser {
    pub fn new<const T: usize, const J: usize>(inputs: [Input; T], expected: [Output; J]) -> Self {
        Self {
            inputs: inputs
                .into_iter()
                .map(NetworkMessage::Input)
                .rev()
                .collect(),
            ouputs: Vec::new(),
            expected: Some(
                expected
                    .into_iter()
                    .map(NetworkMessage::Output)
                    .rev()
                    .collect(),
            ),
        }
    }

    pub fn recive_handshake() -> Self {
        Self::new(
            [Input::Ping, Input::ReplConf, Input::ReplConf, Input::Psync],
            [
                Output::Pong,
                Output::ReplConf,
                Output::ReplConf,
                Output::Psync,
            ],
        )
    }
    pub fn send_handshake() -> Self {
        Self::new(
            [Input::Ping, Input::ReplConf, Input::ReplConf, Input::Psync],
            [
                Output::Pong,
                Output::ReplConf,
                Output::ReplConf,
                Output::Psync,
            ],
        )
    }
}
impl Parser for MockParser {
    fn parse(&mut self, value: rustis::resp::Value) -> anyhow::Result<rustis::io::Input> {
        let Some(next) = self.inputs.pop() else {
            bail!("out of inputs");
        };
        let NetworkMessage::Input(next) = next else {
            panic!();
        };
        Ok(next)
    }

    fn into_value(&mut self, output: rustis::io::Output) -> anyhow::Result<Value> {
        let output = NetworkMessage::Output(output);
        let Some(ref mut expected) = self.expected else {
            self.ouputs.push(output);
            return Ok(Value::SimpleString("UNIMPLIMENTED".into()));
        };
        let expected = expected.pop().expect("got more outputs than expected");
        assert_eq!(output, expected);
        self.ouputs.push(output);
        Ok(Value::SimpleString("UNIMPLIMENTED".into()))
    }

    fn input_into_value(&mut self, input: Input) -> anyhow::Result<Value> {
        let input = NetworkMessage::Input(input);
        let Some(ref mut expected) = self.expected else {
            self.ouputs.push(input);
            return Ok(Value::SimpleString("UNIMPLIMENTED".into()));
        };
        let expected = expected.pop().expect("got more outputs than expected");
        assert_eq!(input, expected);
        self.ouputs.push(input);
        Ok(Value::SimpleString("UNIMPLIMENTED".into()))
    }
}

pub struct TestService;
impl<R, W, E, P> Service<(), R, W, E, P> for TestService
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    type Response = CallResult;

    fn call(
        &mut self,
        _request: (),
        io: &mut rustis::io::Io<R, W, E, P>,
    ) -> anyhow::Result<Self::Response> {
        let request = io.read_input().unwrap();
        match request {
            Input::Ping => io.parser_mut().into_value(rustis::io::Output::Pong),
            Input::Get(_) => todo!(),
            Input::ReplConf => todo!(),
            Input::Psync => todo!(),
            Input::Set {
                key,
                value,
                expiry,
                get,
            } => todo!(),
        }
        .unwrap();
        Ok(CallResult::Ok)
    }
}
