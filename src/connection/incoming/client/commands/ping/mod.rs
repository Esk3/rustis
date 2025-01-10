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

    fn handle(
        &self,
        _input: super::super::Request,
        repo: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        Ok(super::super::Response {
            kind: super::super::ResponseKind::Value(resp::Value::SimpleString("PONG".into())),
            event: None,
        })
    }
}
