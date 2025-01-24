use crate::{command::Command, repository::Repository, Request};

pub struct Ping;

impl Ping {
    fn handle_request(_: PingRequest, _: &Repository) {}
}

impl Command<Request, (), Repository> for Ping {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("PING")
    }

    fn call(&self, request: Request, state: &Repository) -> anyhow::Result<()> {
        Self::handle_request(request.try_into().unwrap(), state);
        Ok(())
    }
}

struct PingRequest;

impl TryFrom<Request> for PingRequest {
    type Error = anyhow::Error;

    fn try_from(_: Request) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}
