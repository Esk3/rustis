use crate::{command::Command, repository::Repository, Request};

pub struct Set;

impl Set {
    fn handle_request(request: SetRequest, repo: &Repository) {
        repo.kv_repo()
            .set(request.key, request.value, None)
            .unwrap();
    }
}

impl Command<Request, (), Repository> for Set {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("SET")
    }

    fn call(&self, request: Request, state: &Repository) -> anyhow::Result<()> {
        Self::handle_request(request.try_into().unwrap(), state);
        Ok(())
    }
}

struct SetRequest {
    key: String,
    value: String,
}

impl TryFrom<Request> for SetRequest {
    type Error = anyhow::Error;

    fn try_from(value: Request) -> Result<Self, Self::Error> {
        let mut args = value.into_standard().unwrap().args;
        let value = args.swap_remove(1);
        let key = args.swap_remove(0);
        Ok(Self { key, value })
    }
}
