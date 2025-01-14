use crate::{
    command::Command,
    repository::{stream_repo::stream::EntryId, Repository},
};

pub struct XRange;

impl XRange {
    fn handle_request(request: Request, repo: &Repository) -> anyhow::Result<Response> {
        repo.stream_repo()
            .xrange(request.stream_key, request.start, request.end);
        todo!()
    }
}

impl Command<super::Request, super::Response, Repository> for XRange {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("XRANGE")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        let request = Request::try_from(request)?;
        Self::handle_request(request, repo).map(std::convert::Into::into)
    }
}

struct Request {
    stream_key: String,
    start: EntryId,
    end: EntryId,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        todo!()
    }
}

struct Response {}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        todo!()
    }
}
