use crate::{node_service::ClientService, resp::decoder::Value};

use super::{
    request,
    response::{self, Response},
};

pub struct Client<S> {
    service: S,
}

impl<S> Client<S>
where
    S: ClientService,
{
    #[must_use]
    pub fn new(service: S) -> Self {
        Self { service }
    }

    pub fn handle_request(&mut self, request: request::Request) -> response::Response {
        match request {
            request::Request::Ping => response::Response::SendPong,
            request::Request::Echo(echo) => self.handle_echo(echo),
            request::Request::Get(key) => self.handle_get(key),
            request::Request::Set { key, value, exp } => self.handle_set(key, value),
        }
    }

    pub fn handle_echo(&self, echo: String) -> Response {
        Response::SendBulkString(echo)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_get(&self, key: impl ToString) -> Response {
        match self.service.get(key.to_string()) {
            Ok(Some(value)) => Response::SendBulkString(value),
            Ok(None) => Response::SendNull,
            Err(()) => panic!(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_set(&self, key: impl ToString, value: impl ToString) -> Response {
        match self.service.set(key.to_string(), value.to_string()) {
            Ok(()) => Response::SendOk,
            Err(()) => todo!(),
        }
    }

    pub fn handle_wait(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_service::{node_worker, tests::dummy_service::AlwaysOk},
        repository::Repository,
    };

    use super::*;

    #[test]
    fn get_always_ok() {
        let c = Client::new(AlwaysOk);
        let response = c.handle_get("abc");
        assert_eq!(
            response,
            Response::SendBulkString("dummy response for key abc".to_string())
        );
    }

    #[test]
    fn empty_get_is_none() {
        let manager = node_worker::run(crate::node::Node, Repository::new());
        let c = Client::new(manager);
        let response = c.handle_get("abc");
        assert_eq!(response, Response::SendNull);
    }

    #[test]
    fn get_some() {
        let manager = node_worker::run(crate::node::Node, Repository::new());
        let c = Client::new(manager);
        let key = "abc";
        let value = "xyz";
        let res = c.handle_set(key.to_string(), value.to_string());
        assert_eq!(res, Response::SendOk);
        let response = c.handle_get(key.to_string());
        assert_eq!(
            response,
            Response::SendBulkString(value.to_string()) //value.to_string()
        );
    }
}
