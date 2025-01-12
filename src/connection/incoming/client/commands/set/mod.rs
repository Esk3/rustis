use anyhow::{bail, Context};

use crate::{command::Command, repository::Repository};

pub struct Set;

impl Set {
    fn handle_request(Request { key, value }: Request, repo: &Repository) -> Response {
        repo.set(key, value, None).unwrap();
        Response
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
        let mut iter = value.value.into_iter();
        assert!(iter
            .next()
            .context("expected set ident")?
            .eq_ignore_ascii_case("SET"));

        let (Some(key), Some(value)) = (iter.next(), iter.next()) else {
            bail!("usage: SET <key> <value>")
        };
        let key = match key {
            crate::resp::Value::SimpleString(s) | crate::resp::Value::BulkString(s) => s,
            crate::resp::Value::SimpleError(_) => todo!(),
            crate::resp::Value::BulkByteString(s) => String::from_utf8_lossy(&s).to_string(),
            crate::resp::Value::NullString => todo!(),
            crate::resp::Value::Integer(_) => todo!(),
            crate::resp::Value::Array(_) => todo!(),
            crate::resp::Value::NullArray => todo!(),
        };
        //let key = key.expect_string()?;
        let value = value.expect_string()?;
        Ok(Self { key, value })
    }
}
struct Response;

impl From<Response> for super::Response {
    fn from(_value: Response) -> Self {
        Self::ok()
    }
}
