use client::ConnectionToClient;
use follower::ConnectionToFollower;

use crate::{
    io::{Encoder, Io, Parser},
    service::Service,
};

pub mod client;
pub mod follower;
pub mod leader;
pub mod services;

#[derive(Debug)]
pub struct Connection<R, W, E, P, S> {
    service: S,
    io: Io<R, W, E, P>,
}

impl<R, W, E, P, S> Connection<R, W, E, P, S>
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
    S: Service<(), R, W, E, P, Response = CallResult>,
{
    pub fn new(service: S, io: Io<R, W, E, P>) -> Self {
        Self { service, io }
    }
}

#[derive(Debug)]
pub enum ConnectionWrapper<R, W, E, P> {
    Client(ConnectionToClient<R, W, E, P>),
    Follower(ConnectionToFollower<R, W, E, P>),
}

impl<R, W, E, P> ConnectionWrapper<R, W, E, P>
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    pub fn call(self) -> anyhow::Result<Self> {
        match self {
            ConnectionWrapper::Client(client) => {
                let res = client.handle_client_request()?;
                match res {
                    client::ClientOption::Client(client) => Ok(Self::Client(client)),
                    client::ClientOption::Manager((io, manager)) => Ok(Self::Follower(
                        ConnectionToFollower::new_connection_to_follower(manager, io),
                    )),
                }
            }
            ConnectionWrapper::Follower(follower) => {
                Ok(Self::Follower(follower.handle_follower_event()))
            }
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            self = self.call()?;
        }
    }

    pub fn steps(mut self, steps: usize) -> anyhow::Result<Self> {
        for _ in 0..steps {
            self = self.call()?;
        }
        Ok(self)
    }
}

pub enum CallResult {
    Ok,
    IntoConnectionToFollower,
}
