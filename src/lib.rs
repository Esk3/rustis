pub mod command;
pub mod config;
pub mod connection;
pub mod event;
pub mod listner;
pub mod message;
pub mod radix;
pub mod redis;
pub mod repository;
pub mod resp;

pub use message::request::Request;
pub use message::Message;

pub trait Service<Req> {
    type Response;
    type Error;
    fn call(&mut self, request: Req) -> Result<Self::Response, Self::Error>;
}
