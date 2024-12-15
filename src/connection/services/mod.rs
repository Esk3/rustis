use super::CallResult;
use crate::{
    io::{Encoder, Input, Io, Output, Parser},
    service::Service,
};

#[derive(Debug)]
pub struct ReadInputService<S> {
    pub inner: S,
}

impl<S, R, W, E, P> Service<(), R, W, E, P> for ReadInputService<S>
where
    S: Service<Input, R, W, E, P>,
    R: std::io::Read,
    E: Encoder,
    P: Parser,
{
    type Response = S::Response;

    fn call(&mut self, _request: (), io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response> {
        let input = io.read_input()?;
        self.inner.call(input, io)
    }
}

#[derive(Debug)]
pub struct ResponseService<S> {
    pub inner: S,
}

impl<Req, S, R, W, E, P> Service<Req, R, W, E, P> for ResponseService<S>
where
    W: std::io::Write,
    E: Encoder,
    P: Parser,
    S: Service<Req, R, W, E, P, Response = Option<Output>>,
{
    type Response = usize;

    fn call(&mut self, request: Req, io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response> {
        let output = self.inner.call(request, io)?;
        let bytes_written = if let Some(output) = output {
            io.write_output(output)?
        } else {
            0
        };
        Ok(bytes_written)
    }
}
