use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Subscribe;
impl Command<super::super::Request, super::super::Response, Repository> for Subscribe {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("SUBSCRIBE")
    }

    fn call(
        &self,
        _: super::super::Request,
        _: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        Ok(super::Response::value(
            resp::Value::bulk_strings("subscribe; __Booksleeve_MasterChanged")
                .into_iter()
                .chain(std::iter::once(resp::Value::Integer(1)))
                .collect(),
        ))
    }
}
