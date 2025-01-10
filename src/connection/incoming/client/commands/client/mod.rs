use crate::{command::Command, repository::Repository, resp};

pub struct Client;
impl Command<super::Request, super::Response, Repository> for Client {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("CLIENT")
    }

    fn call(
        &self,
        _request: super::Request,
        _repo: &Repository,
    ) -> anyhow::Result<super::Response> {
        Ok(super::Response::ok())
    }
}
