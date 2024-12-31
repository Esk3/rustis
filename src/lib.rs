pub mod config;
pub mod connection;
pub mod event;
pub mod listner;
pub mod repository;
pub mod resp;

pub trait Service<Req> {
    type Response;
    type Error;
    fn call(&mut self, request: Req) -> Result<Self::Response, Self::Error>;
}
