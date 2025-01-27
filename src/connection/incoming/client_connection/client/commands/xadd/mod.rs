use crate::{
    command::Command,
    repository::{
        stream_repo::stream::{entry_id::EntryIdKind, EntryId, Field},
        Repository,
    },
    resp,
};

pub struct XAdd;

impl XAdd {
    fn handle_request(request: Request, repo: &Repository) -> Response {
        let stream_repo = repo.stream_repo();
        let key = match request.entry_id {
            EntryIdKind::None(_) => stream_repo.add_auto_increment(
                request.stream_key,
                request.fields,
                &request.timestamp,
            ),
            EntryIdKind::Timestamp(partial_entry_id) => stream_repo
                .add(request.stream_key, partial_entry_id, request.fields)
                .unwrap(),
            EntryIdKind::Full(entry_id) => stream_repo
                .add(request.stream_key, entry_id, request.fields)
                .unwrap(),
        };
        Response::Ok(key)
    }
}

impl Command<super::Request, super::Response, Repository> for XAdd {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("XADD")
    }

    fn call(&self, request: super::Request, repo: &Repository) -> anyhow::Result<super::Response> {
        Ok(Self::handle_request(Request::try_from(request)?, repo).into())
    }
}

struct Request {
    stream_key: String,
    entry_id: EntryIdKind,
    fields: Vec<Field>,
    timestamp: std::time::SystemTime,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let mut iter = value.request.into_standard().unwrap().args.into_iter();
        let stream_key = iter.next().unwrap();
        let entry_id = iter.next().unwrap();
        let timestamp = value.timestamp;

        let mut fields = Vec::new();
        while let Some(name) = iter.next() {
            let value = iter.next().unwrap();
            fields.push(Field::new(name, value));
        }

        let entry_id: EntryIdKind = entry_id.parse().unwrap();
        Ok(Self {
            stream_key,
            entry_id,
            fields,
            timestamp,
        })
    }
}

enum Response {
    Ok(EntryId),
    KeyError,
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        match value {
            Response::Ok(entry_id) => resp::Value::simple_string(entry_id).into(),
            Response::KeyError => todo!(),
        }
    }
}
