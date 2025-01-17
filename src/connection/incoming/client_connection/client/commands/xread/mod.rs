use crate::{
    command::Command,
    repository::{
        stream_repo::{
            stream::{Entry, EntryId},
            BlockResult,
        },
        Repository,
    },
    resp::{self, value::IntoRespArray},
};

pub struct XRead;

impl XRead {
    fn handle_request(request: Request, repo: &Repository) -> Response {
        let count = request.count.unwrap_or(1);
        let entries = if let Some(block_duration) = request.block {
            // TODO blocking does not work on mutliple streams
            request
                .streams
                .into_iter()
                .map(
                    |Stream {
                         stream_key,
                         entry_id,
                     }| {
                        let entries = repo.stream_repo().read_blocking(
                            stream_key.clone(),
                            &entry_id,
                            count,
                            block_duration,
                        );
                        StreamResponse::new(
                            stream_key,
                            match entries {
                                BlockResult::Found(entries) => entries,
                                BlockResult::NotFound => Vec::new(),
                                BlockResult::Err(_) => todo!(),
                            },
                        )
                    },
                )
                .collect::<Vec<_>>()
        } else {
            request
                .streams
                .into_iter()
                .map(
                    |Stream {
                         stream_key,
                         entry_id,
                     }| {
                        let entries = repo
                            .stream_repo()
                            .read(stream_key.clone(), &entry_id, count)
                            .unwrap();
                        Ok(StreamResponse::new(stream_key, entries))
                    },
                )
                .collect::<Result<Vec<_>, anyhow::Error>>()
                .unwrap()
        };
        Response::Ok(entries)
    }
}

impl Command<super::Request, super::Response, Repository> for XRead {
    fn info(&self) -> crate::command::CommandInfo {
        crate::command::CommandInfo::new_name("XREAD")
    }

    fn call(&self, request: super::Request, state: &Repository) -> anyhow::Result<super::Response> {
        Ok(Self::handle_request(request.try_into().unwrap(), state).into())
    }
}

struct Request {
    count: Option<usize>,
    block: Option<Option<std::time::Duration>>,
    streams: Vec<Stream>,
}

impl TryFrom<super::Request> for Request {
    type Error = anyhow::Error;

    fn try_from(value: super::Request) -> Result<Self, Self::Error> {
        let content = value.into_content().unwrap();
        let mut args = content.len();
        let mut iter = content.into_iter().peekable();
        let count = if iter.peek().unwrap().eq_ignore_ascii_case("COUNT") {
            iter.next();
            args -= 2;
            Some(iter.next().unwrap().parse().unwrap())
        } else {
            None
        };

        let block = if iter.peek().unwrap().eq_ignore_ascii_case("BLOCK") {
            iter.next();
            args -= 2;
            let millis = iter.next().unwrap().parse().unwrap();
            if millis == 0 {
                Some(None)
            } else {
                Some(Some(std::time::Duration::from_millis(millis)))
            }
        } else {
            None
        };

        assert!(iter.next().unwrap().eq_ignore_ascii_case("STREAMS"));
        args -= 1;

        assert_eq!(args % 2, 0);
        let streams = args / 2;
        let (stream_keys, entry_ids): (Vec<_>, Vec<_>) =
            iter.enumerate().partition(|(i, _)| *i < streams);
        let streams = stream_keys
            .into_iter()
            .zip(entry_ids)
            .map(|((_, key), (_, id))| Stream::new(key, &id))
            .collect();

        Ok(Self {
            count,
            block,
            streams,
        })
    }
}

struct Stream {
    stream_key: String,
    entry_id: EntryId,
}

impl Stream {
    fn new(stream_key: String, entry_id: &str) -> Self {
        let id = if entry_id == "*" {
            EntryId::min()
        } else if let Some((timestamp, id)) = entry_id.split_once('-') {
            EntryId::new(timestamp.parse().unwrap(), id.parse().unwrap())
        } else {
            EntryId::new(entry_id.parse().unwrap(), 0)
        };
        Self {
            stream_key,
            entry_id: id,
        }
    }
}

struct StreamResponse {
    stream_key: String,
    entries: Vec<Entry>,
}

impl StreamResponse {
    fn new(stream_key: String, entries: Vec<Entry>) -> Self {
        Self {
            stream_key,
            entries,
        }
    }
}

impl From<StreamResponse> for resp::Value {
    fn from(value: StreamResponse) -> Self {
        [
            resp::Value::simple_string(value.stream_key),
            value
                .entries
                .into_iter()
                .map(std::convert::Into::into)
                .collect(),
        ]
        .into_array()
    }
}

enum Response {
    Ok(Vec<StreamResponse>),
}

impl From<Response> for super::Response {
    fn from(value: Response) -> Self {
        match value {
            Response::Ok(responses) => Self::value(
                responses
                    .into_iter()
                    .map(std::convert::Into::into)
                    .collect(),
            ),
        }
    }
}
