use crate::{
    command::{parser::Parser, Command},
    repository::Repository,
    resp::{self, value::IntoRespArray},
};

pub struct Config;

impl Config {
    fn handle_request(request: Request) -> Response {
        let value = match request.key.to_lowercase().as_str() {
            "databases" => "16",
            "slave-read-only" => "yes",
            _ => todo!("{}", request.key),
        }
        .to_string();
        Response {
            key: request.key,
            value,
        }
    }
}
impl Command<super::Request, super::Response, Repository> for Config {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("CONFIG")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        Ok(Self::handle_request(request.try_into().unwrap()).into())
    }
}

struct Request {
    key: String,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let key = Parser::new(value.value)
            .ident("CONFIG")
            .unwrap()
            .ident("GET")
            .unwrap()
            .value("key")
            .unwrap()
            .finish()
            .remove("key")
            .unwrap();
        Ok(Self { key })
    }
}

struct Response {
    key: String,
    value: String,
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        let res = [
            resp::Value::bulk_string(value.key),
            resp::Value::bulk_string(value.value),
        ]
        .to_vec();
        return Self::value(res.into_array());
    }
}
