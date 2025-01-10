use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
};

pub struct Echo;
impl Command<super::super::Request, super::super::Response, Repository> for Echo {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("ECHO")
    }

    fn handle(
        &self,
        input: super::super::Request,
        repo: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        todo!()
    }
}
