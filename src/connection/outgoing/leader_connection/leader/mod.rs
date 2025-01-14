use crate::Request;
use crate::{
    event::{self, EventEmitter},
    repository::Repository,
    resp,
};

mod response;

#[cfg(test)]
mod tests;

pub use response::Response;

pub struct Leader {
    router: (),
    repo: Repository,
    emitter: EventEmitter,
}

impl Leader {
    pub fn new(router: (), emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            router,
            repo,
            emitter,
        }
    }

    pub fn handle_request(&mut self, request: Request) -> anyhow::Result<Option<resp::Value>> {
        let Request::Standard(request) = request;
        self.repo
            .kv_repo()
            .set("MyKey".to_string(), "myValue".to_string(), None)
            .unwrap();
        self.emitter.emmit(event::Kind::Set {
            key: "MyKey".to_string(),
            value: "myValue".to_string(),
            expiry: None,
        });
        match request.command.to_uppercase().as_str() {
            "SET" => Ok(None),
            _ => todo!(),
        }
    }
}
