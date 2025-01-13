use crate::{
    command::{parser::Parser, Command, CommandInfo},
    repository::Repository,
    resp,
};

pub struct Info;

impl Info {
    fn handle_request(request: Request) -> Response {
        let s = match request.key.to_lowercase().as_str() {
            "replication" => {
                "# Replication\r\nrole:master\r\nconnected_slaves:0\r\nmaster_failover_state:no-failover\r\nmaster_replid:5e7a4edcb3a968adab189dfed97a982463d347a5\r\nmaster_repl_offset:0\r\nsecond_repl_offset:-1\r\nrepl_backlog_active:0\r\nrepl_backlog_size:1048576\r\nrepl_backlog_first_byte_offset:0\r\nrepl_backlog_histlen:0\r\n"
            }

            "server" => {
                "# Server\r\nredis_version:7.2.6\r\nredis_git_sha1:00000000\r\nredis_git_dirty:0\r\nredis_build_id:bb0ef6c2693a5dd0\r\nredis_mode:standalone\r\nos:Linux 6.6.64 x86_64\r\narch_bits:64\r\nmonotonic_clock:POSIX clock_gettime\r\nmultiplexing_api:epoll\r\natomicvar_api:c11-builtin\r\ngcc_version:13.3.0\r\nprocess_id:96237\r\nprocess_supervised:no\r\nrun_id:16306cd80d0b82429a5f6a0cc2fc7d72b0c635bd\r\ntcp_port:6379\r\nserver_time_usec:1736605053230053\r\nuptime_in_seconds:5\r\nuptime_in_days:0\r\nhz:10\r\nconfigured_hz:10\r\nlru_clock:8551805\r\nexecutable:/home/eske/redis-server\r\nconfig_file:\r\nio_threads_active:0\r\nlistener0:name=tcp,bind=*,bind=-::*,port=6379\r\n"
            }
            _ => todo!(),
        }
        .to_string();
        Response(s)
    }
}

impl Command<super::super::Request, super::super::Response, Repository> for Info {
    fn info(&self) -> CommandInfo {
        CommandInfo::new_name("INFO")
    }

    fn call(
        &self,
        request: super::super::Request,
        _: &Repository,
    ) -> anyhow::Result<super::super::Response> {
        Ok(Self::handle_request(request.try_into().unwrap()).into())
    }
}

struct Request {
    key: String,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let key = Parser::new(value.value)
            .ident("info")
            .unwrap()
            .value("key")
            .unwrap()
            .finish()
            .remove("key")
            .unwrap();
        Ok(Self { key })
    }
}

struct Response(String);
impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        Self::value(resp::Value::bulk_string(value.0))
    }
}
