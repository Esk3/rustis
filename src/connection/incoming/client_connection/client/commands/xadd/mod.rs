use crate::{
    command::Command,
    repository::{
        stream_repo::stream::{entry_id, EntryId, Field},
        Repository,
    },
    resp,
};

pub struct XAdd;

impl XAdd {
    fn handle_request(request: Request, repo: &Repository) -> Response {
        let stream_repo = repo.stream_repo();
        let key = match request.entry_id {
            EntryIdKind::None(_) => stream_repo.xadd_auto_increment(
                request.stream_key,
                request.fields,
                &request.timestamp,
            ),
            EntryIdKind::Timestamp(partial_entry_id) => {
                todo!()
                //stream_repo
                //            .xadd(request.stream_key, partial_entry_id, request.fields)
                //            .unwrap()
            }
            EntryIdKind::Full(entry_id) => {
                todo!()
                //stream_repo
                //            .xadd(request.stream_key, entry_id, request.fields)
                //            .unwrap()
            }
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
        // TODO should be field then value
        let mut fields = Vec::new();
        while let Some(name) = iter.next() {
            let value = iter.next().unwrap();
            fields.push(Field::new(name, value));
        }

        let entry_id = match entry_id.as_str() {
            "*" => EntryIdKind::None(entry_id::EmptyEntryId),
            timestamp if timestamp.ends_with("-*") => {
                let timestamp = timestamp.split('-').next().unwrap();
                EntryIdKind::Timestamp(entry_id::TimestampEntryId::from_millis(
                    timestamp.parse().unwrap(),
                ))
            }
            full => {
                let (timestamp, id) = full.split_once('-').unwrap();
                EntryIdKind::Full(entry_id::EntryId::new(
                    timestamp.parse().unwrap(),
                    id.parse().unwrap(),
                ))
            }
        };
        Ok(Self {
            stream_key,
            entry_id,
            fields,
            timestamp,
        })
    }
}

enum EntryIdKind {
    None(entry_id::EmptyEntryId),
    Timestamp(entry_id::TimestampEntryId),
    Full(entry_id::EntryId),
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
