use crate::{
    event::{self, EventEmitter},
    radix::Radix,
    repository::Repository,
    resp,
};

pub mod echo;
pub mod ping;

pub struct Request {
    value: Vec<resp::Value>,
    size: usize,
    timestamp: std::time::SystemTime,
}

pub enum ResponseKind {
    Value(resp::Value),
    RecivedReplConf(resp::Value),
}

pub struct Response {
    kind: ResponseKind,
    event: Option<event::Kind>,
}

pub struct Client {
    router: &'static CommandRouter,
}

impl Client {
    pub fn new(router: &'static CommandRouter, event: EventEmitter, repo: Repository) -> Self {
        Self { router }
    }
    pub fn default_router() -> &'static CommandRouter {
        let mut router = CommandRouter::new();
        router.add(ping::Ping).add(echo::Echo);
        Box::leak(Box::new(router))
    }
    pub fn handle_request(&self, request: Request) -> anyhow::Result<Response> {
        let handler = self
            .router
            .route(request.value[0].clone().expect_string().unwrap().as_bytes())
            .unwrap();
        handler.handle(request)
    }
}

trait Command {
    fn info(&self) -> CommandInfo;
    fn handle(&self, input: Request) -> anyhow::Result<Response>;
}

pub struct CommandInfo {
    name: String,
}

struct Helper {
    state: (),
}

pub struct CommandRouter {
    routes: Radix<Box<dyn Command>>,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            routes: Radix::new(),
        }
    }
    pub fn add<C>(&mut self, command: C) -> &mut Self
    where
        C: Command + 'static,
    {
        self.routes
            .add(command.info().name.as_bytes(), Box::new(command));
        self
    }
    pub fn route(&self, cmd: &[u8]) -> Option<&dyn Command> {
        self.routes.get(cmd).map(|cmd| &**cmd)
    }
}
