use crate::{
    command::{Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Info;
impl Command<super::super::Request, super::super::Response, Repository> for Info {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("INFO")
    }

    fn call(
        &self,
        _: super::super::Request,
        _: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        let response = Response(String::from(
            "# Replication
role:master
connected_slaves:0
master_failover_state:no-failover
master_replid:5e7a4edcb3a968adab189dfed97a982463d347a5
master_replid2:0000000000000000000000000000000000000000
master_repl_offset:0
second_repl_offset:-1
repl_backlog_active:0
repl_backlog_size:1048576
repl_backlog_first_byte_offset:0
repl_backlog_histlen:0",
        ));
        Ok(response.into())
    }
}

struct Response(String);
impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        Self::value(resp::Value::bulk_string(value.0))
    }
}
