use crate::{
    io::{Input, Output},
    message_broker::{
        manager::{Manager, WorkerManager},
        message,
    },
    service::Service,
};

use super::{
    services::{ParseService, ReadInputService},
    Connection,
};

pub type ConnectionToClientService =
    ReadInputService<ParseService<IntoConnectionToFollowerService<ClientService>>>;

impl ConnectionToClientService {
    pub fn new(manager: WorkerManager) -> Self {
        Self {
            inner: ParseService {
                inner: IntoConnectionToFollowerService {
                    inner: ClientService { manager },
                },
            },
        }
    }
    pub fn into_manager(self) -> WorkerManager {
        self.inner.inner.inner.manager
    }
}

#[derive(Debug)]
pub struct IntoConnectionToFollowerService<S> {
    pub inner: S,
}

impl<S> Service<Input> for IntoConnectionToFollowerService<S>
where
    S: Service<Input>,
{
    type Response = Kind<S::Response>;

    fn call(&mut self, request: Input) -> anyhow::Result<Self::Response> {
        match request {
            Input::Psync => todo!("res"),
            _ => {
                let res = self.inner.call(request)?;
                Ok(Kind::Response(res))
            }
        }
    }
}

#[derive(Debug)]
pub enum Kind<T> {
    Response(T),
    IntoFollower,
}

#[derive(Debug)]
pub struct ClientService {
    pub manager: WorkerManager,
}

impl Service<Input> for ClientService {
    type Response = Option<Output>;

    fn call(&mut self, request: Input) -> anyhow::Result<Self::Response> {
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
                self.manager
                    .send(message::Request::Set {
                        key,
                        value,
                        expiry,
                        get,
                    })
                    .unwrap();
                Ok(Some(Output::Set))
            }
        }
    }
}
