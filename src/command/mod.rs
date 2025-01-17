use crate::radix::Radix;

pub mod parser;

pub trait Command<Req, Res, S>: Sync {
    fn info(&self) -> CommandInfo;
    fn call(&self, request: Req, state: &S) -> anyhow::Result<Res>;
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            routes: Radix::new(),
        }
    }
    pub fn add<C>(&mut self, command: C) -> &mut Self
    where
        C: Command<Req, Res, S> + 'static,
    {
        if self
            .routes
            .add(
                command.info().name.to_uppercase().as_bytes(),
                Box::new(command),
            )
            .is_err()
        {
            panic!("error adding command to router")
        };
        self
    }

    #[must_use]
    pub fn route(&self, cmd: &[u8]) -> Option<&dyn Command<Req, Res, S>> {
        self.routes.get(&cmd.to_ascii_uppercase()).map(|cmd| &**cmd)
    }
}

impl<Req, Res, S> Default for CommandRouter<Req, Res, S> {
    fn default() -> Self {
        Self::new()
    }
}
