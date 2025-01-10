use crate::{
    command::Command, connection::incoming::client::ResponseKind, repository::Repository, resp,
};

pub struct Client;
impl Command<super::Request, super::Response, Repository> for Client {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("CLIENT")
    }

    fn handle(&self, request: super::Request, repo: Repository) -> anyhow::Result<super::Response> {
        Ok(super::Response {
            kind: ResponseKind::Value(resp::Value::SimpleString("OK".into())),
            event: None,
        })
    }
}
