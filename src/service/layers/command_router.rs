use anyhow::bail;

use crate::{command::CommandRouter as Router, service::Service};

pub struct CommandRouter<Req, Res, State>
where
    Req: 'static,
    Res: 'static,
    State: 'static,
{
    router: &'static Router<Req, Res, State>,
    state: State,
}

impl<Req, Res, State> CommandRouter<Req, Res, State>
where
    Req: Routeable,
{
    #[must_use]
    pub fn new(state: State, router: &'static Router<Req, Res, State>) -> Self {
        Self { router, state }
    }

    fn handler(&self, request: &Req) -> Option<&dyn crate::command::Command<Req, Res, State>> {
        self.router.route(&request.route_name())
    }
}

impl<Req, Res, State> Service<Req> for CommandRouter<Req, Res, State>
where
    Req: Routeable,
{
    type Response = Res;

    type Error = anyhow::Error;

    fn call(&mut self, request: Req) -> Result<Self::Response, Self::Error> {
        let Some(handler) = self.handler(&request) else {
            bail!("handler not found");
        };
        handler.call(request, &self.state)
    }
}

pub trait Routeable {
    fn route_name(&self) -> Vec<u8>;
}
