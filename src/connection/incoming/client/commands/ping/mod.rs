use crate::resp;

pub struct Ping;
impl super::Command for Ping {
    fn info(&self) -> super::CommandInfo {
        super::CommandInfo::new_name("PING")
    }

    fn handle(&self, _input: super::super::Request) -> anyhow::Result<super::super::Response> {
        Ok(super::super::Response {
            kind: super::super::ResponseKind::Value(resp::Value::SimpleString("PONG".into())),
            event: None,
        })
    }
}
