use anyhow::Context;

use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Get;

impl Get {
    fn handle_command(Request { key, timestamp }: Request, repo: &Repository) -> Response {
        repo.kv_repo().get(&key, timestamp).unwrap().into()
    }
}
impl Command<super::Request, super::Response, Repository> for Get {
    fn info(&self) -> crate::command::CommandInfo {
        CommandInfo::new_name("GET")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        Ok(Self::handle_command(request.try_into()?, repo).into())
    }
}

struct Request {
    key: String,
    timestamp: std::time::SystemTime,
}
impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let timestamp = value.timestamp;
        let mut iter = value.into_content().unwrap().into_iter();
        let key = iter.next().context("key missing")?;
        Ok(Self { key, timestamp })
    }
}

enum Response {
    Value(String),
    Null,
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        match value {
            Response::Value(value) => resp::Value::simple_string(value),
            Response::Null => resp::Value::NullString,
        }
        .into()
    }
}

impl From<Option<String>> for Response {
    fn from(value: Option<String>) -> Self {
        if let Some(value) = value {
            Self::Value(value)
        } else {
            Self::Null
        }
    }
}
