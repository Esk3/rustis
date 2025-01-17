use crate::{command::Command, repository::Repository};

pub struct Select;
impl Command<super::Request, super::Response, Repository> for Select {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("SELECT")
    }

    fn call(&self, request: super::Request, state: &Repository) -> anyhow::Result<super::Response> {
        tracing::warn!("SELECT not implimented");
        Ok(super::Response::ok())
    }
}
