use crate::{command::Command, repository::Repository, resp};

pub struct Config;
impl Command<super::Request, super::Response, Repository> for Config {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("CONFIG")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        let key = "abc";
        let value = "xyz";
        let expiry = None;
        repo.set(key.into(), value.into(), expiry).unwrap();
        Ok(super::Response::ok())
    }
}
