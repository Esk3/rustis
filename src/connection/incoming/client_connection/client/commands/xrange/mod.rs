use crate::{
    command::Command,
    repository::{
        stream_repo::stream::{entry_id::EntryIdKind, Entry, EntryId, PartialEntryId},
        Repository,
    },
};

pub struct XRange;

impl XRange {
    fn handle_request(request: Request, repo: &Repository) -> anyhow::Result<Response> {
        repo.stream_repo()
            .range(request.stream_key, &request.start, &request.end)
            .map(Response::new)
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
    count: Option<usize>,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let mut iter = value.into_content().unwrap().into_iter();
        let key = iter.next().unwrap();
        let start: EntryIdKind = iter.next().unwrap().parse().unwrap();
        let end: EntryIdKind = iter.next().unwrap().parse().unwrap();
        let start = start.into_entry_id_or_default(&EntryId::new(0, 0));
        let end = end.into_entry_id_or_default(&EntryId::new(0, 0));
        let count = iter.next().map(|count| count.parse().unwrap());
        Ok(Self {
            stream_key: key,
            start,
            end,
            count,
        })
    }
}

struct Response(Vec<Entry>);

impl Response {
    fn new(entries: Vec<Entry>) -> Self {
        Self(entries)
    }
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        Self::value(value.0.into_iter().map(std::convert::Into::into).collect())
    }
}
