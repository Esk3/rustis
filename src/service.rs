use crate::io::{Encoder, Io, Parser};

pub trait Service<Req, R, W, E, P> {
    type Response;

    fn call(&mut self, request: Req, io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response>;
}
