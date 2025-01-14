use crate::connection::incoming::client_connection::client::{Request, Response, Router};
use crate::{repository::Repository, resp, Service};

pub struct Routing {
    pub repo: Repository,
    pub router: &'static Router,
}

impl Routing {
    #[must_use]
    pub fn new(repo: Repository, router: &'static Router) -> Self {
        Self { repo, router }
    }

    fn handler(
        &self,
        request: &Request,
    ) -> Option<&dyn crate::command::Command<Request, Response, Repository>> {
        self.router.route(request.command()?.as_bytes())
    }
}

impl Service<Request> for Routing {
    type Response = Response;

    type Error = anyhow::Error;

    fn call(&mut self, request: Request) -> Result<Self::Response, Self::Error> {
        let Some(handler) = self.handler(&request) else {
            // TODO handle routing failed somewhere else
            tracing::warn!("unknown command {:?}", request.command().unwrap());
            return Ok(Response::value(resp::Value::SimpleError(
                "ERR unknown command 'SENTINEL', with args beginning with: 'masters'".into(),
            )));
        };
        handler.call(request, &self.repo)
    }
}
