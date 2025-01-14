use anyhow::{bail, Context};

use crate::{command::Command, repository::Repository, resp};

pub struct Set;

impl Set {
    fn handle_request(Request { key, value }: Request, repo: &Repository) -> Response {
        repo.kv_repo()
            .set(key.clone(), value.clone(), None)
            .unwrap();
        Response { key, value }
    }
}
impl Command<super::Request, super::Response, Repository> for Set {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("SET")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        Ok(Self::handle_request(request.try_into()?, repo).into())
    }
}

struct Request {
    key: String,
    value: String,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let mut iter = value.into_content().unwrap().into_iter();

        let (Some(key), Some(value)) = (iter.next(), iter.next()) else {
            bail!("usage: SET <key> <value>")
        };
        Ok(Self { key, value })
    }
}

struct Response {
    key: String,
    value: String,
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        Self::value_event(
            resp::Value::ok(),
            crate::event::Kind::Set {
                key: value.key,
                value: value.value,
                expiry: None,
            },
        )
    }
}
