use std::sync::Mutex;

use crate::{command::Command, repository::Repository, resp};

static ID: Mutex<usize> = Mutex::new(1);

pub struct Client;

impl Client {
    fn handle_request(request: Request) -> Response {
        match request.cmd {
            Cmd::Other => Response::Ok,
            Cmd::Id => {
                let mut lock = ID.lock().unwrap();
                *lock += 1;
                Response::Id(*lock)
            }
        }
    }
}
impl Command<super::Request, super::Response, Repository> for Client {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("CLIENT")
    }

    fn call(&self, request: super::Request, _repo: &Repository) -> anyhow::Result<super::Response> {
        Ok(Self::handle_request(request.try_into().unwrap()).into())
    }
}

struct Request {
    cmd: Cmd,
}

enum Cmd {
    Other,
    Id,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let mut iter = value.value.into_iter();
        _ = iter.next();
        let sub_cmd = iter.next().unwrap().expect_string().unwrap();
        if sub_cmd.eq_ignore_ascii_case("ID") {
            Ok(Request { cmd: Cmd::Id })
        } else {
            Ok(Request { cmd: Cmd::Other })
        }
    }
}

enum Response {
    Ok,
    Id(usize),
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        match value {
            Response::Ok => Self::ok(),
            Response::Id(id) => Self::value(resp::Value::Integer(id.try_into().unwrap())),
        }
    }
}
