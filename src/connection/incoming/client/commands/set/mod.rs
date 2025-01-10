use crate::{command::Command, repository::Repository, resp};

pub struct Set;
impl Command<super::Request, super::Response, Repository> for Set {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("SET")
    }

    fn handle(&self, request: super::Request, repo: Repository) -> anyhow::Result<super::Response> {
        let key = "abc";
        let value = "xyz";
        let expiry = None;
        repo.set(key.into(), value.into(), expiry).unwrap();
        Ok(super::Response {
            kind: crate::connection::incoming::client::ResponseKind::Value(
                resp::Value::SimpleString("OK".into()),
            ),
            event: None,
        })
    }
}
