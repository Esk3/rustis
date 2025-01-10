use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Ping;
impl Command<super::super::Request, super::super::Response, Repository> for Ping {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("PING")
    }

    fn call(
        &self,
        _: super::super::Request,
        _: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        Ok(resp::Value::simple_string("PONG").into())
    }
}
