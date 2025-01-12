use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Cluster;
impl Command<super::super::Request, super::super::Response, Repository> for Cluster {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("CLUSTER")
    }

    fn call(
        &self,
        _: super::super::Request,
        _: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        Ok(
            resp::Value::SimpleError("ERR This instance has cluster support disabled ".into())
                .into(),
        )
    }
}
