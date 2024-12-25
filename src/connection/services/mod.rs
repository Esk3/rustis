use crate::{
    io::{Input, Io, Output},
    resp::Value,
    service::Service,
};

use super::client::Kind;

#[derive(Debug)]
pub struct ReadInputService<S> {
    pub inner: S,
}

impl<S, IO> Service<&mut IO> for ReadInputService<S>
where
    S: Service<Value>,
    IO: Io,
{
    type Response = S::Response;

    fn call(&mut self, io: &mut IO) -> anyhow::Result<Self::Response> {
        let input = io.read_value()?;
        self.inner.call(input)
    }
}

#[derive(Debug)]
pub struct ParseService<S> {
    pub inner: S,
}

impl<S> Service<Value> for ParseService<S>
where
    S: Service<Input, Response = Kind<Option<Output>>>,
{
    type Response = Kind<Value>;

    fn call(&mut self, value: Value) -> anyhow::Result<Self::Response> {
        let output = self.inner.call(Input::Ping).unwrap();
        Ok(Kind::Response(Value::BulkString("abc".into())))
    }
}
