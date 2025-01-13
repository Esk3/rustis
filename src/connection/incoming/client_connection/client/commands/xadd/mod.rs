use crate::{
    command::Command,
    repository::{stream_repo::StreamId, Repository},
    resp,
};

pub struct XAdd;

impl XAdd {
    fn handle_request(request: Request, repo: &Repository) -> Response {
        let key = repo
            .stream_repo()
            .xadd(request.stream_key, request.entry_id, request.value)
            .unwrap();
        //Ok(resp::Value::simple_string(key).into());
        todo!()
    }
}

impl Command<super::Request, super::Response, Repository> for XAdd {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("XADD")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        let request = Request::try_from(request.value)?;
        Self::handle_request(request, repo);
        todo!()
    }
}

struct Request {
    stream_key: String,
    entry_id: Option<StreamId>,
    value: String,
}
impl TryFrom<Vec<resp::Value>> for Request {
    type Error = anyhow::Error;

    fn try_from(value: Vec<resp::Value>) -> Result<Self, Self::Error> {
        todo!()
    }
}
struct Response {}
