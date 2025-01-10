use crate::radix::Radix;

pub trait Command<Req, Res, S>: Sync {
    fn info(&self) -> CommandInfo;
    fn handle(&self, request: Req, state: S) -> anyhow::Result<Res>;
}

pub struct CommandInfo {
    name: String,
}

impl CommandInfo {
    pub fn new_name(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub struct CommandRouter<Req, Res, S> {
    routes: Radix<Box<dyn Command<Req, Res, S>>>,
}

impl<Req, Res, S> CommandRouter<Req, Res, S> {
    pub fn new() -> Self {
        Self {
            routes: Radix::new(),
        }
    }
    pub fn add<C>(&mut self, command: C) -> &mut Self
    where
        C: Command<Req, Res, S> + 'static,
    {
        self.routes.add(
            command.info().name.to_uppercase().as_bytes(),
            Box::new(command),
        );
        self
    }

    pub fn route(&self, cmd: &[u8]) -> Option<&dyn Command<Req, Res, S>> {
        self.routes.get(&cmd.to_ascii_uppercase()).map(|cmd| &**cmd)
    }
}
