use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Echo;

impl Echo {
    fn handle_request(request: Request) -> Response {
        Response { echo: request.echo }
    }
}

impl Command<super::super::Request, super::super::Response, Repository> for Echo {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("ECHO")
    }

    fn call(
        &self,
        request: super::super::Request,
        _repo: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        Ok(Self::handle_request(request.try_into().unwrap()).into())
    }
}

struct Request {
    echo: Vec<u8>,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let mut iter = value.into_content().unwrap().into_iter();

        let value = iter.next().unwrap();

        let echo = value.as_bytes().to_vec();
        Ok(Self { echo })
    }
}

struct Response {
    echo: Vec<u8>,
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        Self::value(resp::Value::BulkByteString(value.echo))
    }
}
