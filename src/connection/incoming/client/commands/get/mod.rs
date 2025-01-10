use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Get;
impl Command<super::Request, super::Response, Repository> for Get {
    fn info(&self) -> crate::command::CommandInfo {
        CommandInfo::new_name("GET")
    }

    fn handle(&self, request: super::Request, repo: Repository) -> anyhow::Result<super::Response> {
        let key = "abc";
        let timestamp = std::time::SystemTime::UNIX_EPOCH;
        let value = repo.kv_repo().get(key, timestamp).unwrap();
        let value = match value {
            Some(value) => resp::Value::SimpleString(value),
            None => resp::Value::NullArray,
        };
        Ok(super::Response {
            kind: crate::connection::incoming::client::ResponseKind::Value(value),
            event: None,
        })
    }
}
