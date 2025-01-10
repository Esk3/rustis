use crate::{command::CommandRouter, repository::Repository};

use super::{Request, Response};

pub struct Router(CommandRouter<Request, Response, Repository>);

impl Router {
    #[must_use]
    pub fn new() -> Self {
        Self(CommandRouter::new())
    }
    pub fn add<C>(&mut self, command: C) -> &mut CommandRouter<Request, Response, Repository>
    where
        C: crate::command::Command<Request, Response, Repository> + 'static,
    {
        self.0.add(command)
    }

    pub fn route(
        &self,
        cmd: &[u8],
    ) -> Option<&dyn crate::command::Command<Request, Response, Repository>> {
        self.0.route(cmd)
    }
}

#[must_use]
pub fn default_router() -> &'static Router {
    let mut router = Router::new();
    router
        .add(super::commands::ping::Ping)
        .add(super::commands::echo::Echo)
        .add(super::commands::get::Get)
        .add(super::commands::set::Set)
        .add(super::commands::xadd::XAdd)
        .add(super::commands::client::Client)
        .add(super::commands::config::Config)
        .add(super::commands::xrange::XRange);
    Box::leak(Box::new(router))
}
