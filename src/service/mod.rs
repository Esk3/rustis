pub mod layers;

pub trait Service<Req> {
    type Response;
    type Error;
    fn call(&mut self, request: Req) -> Result<Self::Response, Self::Error>;
}
