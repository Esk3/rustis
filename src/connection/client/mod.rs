use crate::{
    io::{Encoder, Input, Io, Output, Parser},
    message_broker::{
        manager::{Manager, WorkerManager},
        message,
    },
    service::Service,
};

use super::{
    services::{ReadInputService, ResponseService},
    CallResult, Connection,
};

pub type ConnectionToClient<R, W, E, P, M = WorkerManager> = Connection<
    R,
    W,
    E,
    P,
    ReadInputService<IntoConnectionToFollowerService<ResponseService<ClientService<M>>>>,
>;

#[derive(Debug)]
pub struct IntoConnectionToFollowerService<S> {
    pub inner: S,
}

impl<S, R, W, E, P> Service<Input, R, W, E, P> for IntoConnectionToFollowerService<S>
where
    S: Service<Input, R, W, E, P>,
{
    type Response = CallResult;

    fn call(&mut self, request: Input, io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response> {
        match request {
            Input::Psync => Ok(CallResult::IntoConnectionToFollower),
            _ => {
                self.inner.call(request, io)?;
                Ok(CallResult::Ok)
            }
        }
    }
}

#[derive(Debug)]
pub struct ClientService<M> {
    pub manager: M,
}

impl<M, R, W, E, P> Service<Input, R, W, E, P> for ClientService<M>
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
    M: Manager,
{
    type Response = Option<Output>;

    fn call(&mut self, request: Input, io: &mut Io<R, W, E, P>) -> anyhow::Result<Self::Response> {
        match request {
            Input::Ping => Ok(Some(Output::Pong)),
            Input::Get(_) => todo!(),
            Input::ReplConf => Ok(Some(Output::ReplConf)),
            Input::Psync => Ok(Some(Output::Psync)),
            Input::Set {
                key,
                value,
                expiry,
                get,
            } => {
                self.manager.send(message::Request::Set {
                    key,
                    value,
                    expiry,
                    get,
                });
                Ok(Some(Output::Set))
            }
        }
    }
}

impl<R, W, E, P, M> ConnectionToClient<R, W, E, P, M>
where
    R: std::io::Read,
    W: std::io::Write,
    E: Encoder,
    P: Parser,
{
    #[must_use]
    pub fn new_connection_to_client(manager: M, io: Io<R, W, E, P>) -> Self {
        Self {
            service: ReadInputService {
                inner: IntoConnectionToFollowerService {
                    inner: ResponseService {
                        inner: ClientService { manager },
                    },
                },
            },

            io,
        }
    }

    pub fn handle_client_request(mut self) -> anyhow::Result<ClientOption<R, W, E, P, M>>
    where
        M: Manager,
    {
        match self.service.call((), &mut self.io).unwrap() {
            CallResult::Ok => Ok(ClientOption::Client(self)),
            CallResult::IntoConnectionToFollower => {
                let manager = self.into_manager();
                Ok(ClientOption::Manager(manager))
            }
        }
    }
    pub fn into_manager(self) -> (Io<R, W, E, P>, M) {
        (self.io, self.service.inner.inner.inner.manager)
    }
}

#[derive(Debug)]
pub enum ClientOption<R, W, E, P, M> {
    Client(ConnectionToClient<R, W, E, P, M>),
    Manager((Io<R, W, E, P>, M)),
}
