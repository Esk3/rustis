use crate::resp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Request {
    Standard(Standard),
}

impl Request {
    #[must_use]
    pub fn command(&self) -> Option<&str> {
        match self {
            Request::Standard(s) => Some(s.command.as_str()),
        }
    }

    pub fn into_standard(self) -> Result<Standard, Self> {
        match self {
            Self::Standard(s) => Ok(s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Standard {
    pub command: String,
    pub args: Vec<String>,
}

impl Standard {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<I, T>(command: impl ToString, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        Self {
            command: command.to_string(),
            args: args.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn new_empty(command: impl ToString) -> Self {
        Self::new(command, None::<String>)
    }
}

impl From<Standard> for Request {
    fn from(value: Standard) -> Self {
        Self::Standard(value)
    }
}

impl From<super::Message<resp::Value>> for Request {
    fn from(value: super::Message<resp::Value>) -> Self {
        match value.content.into_array() {
            Ok(arr) => {
                if let Some(mut req) = arr.iter().all(resp::value::Value::is_string).then(|| {
                    arr.into_iter()
                        .map(|v| v.expect_string().unwrap())
                        .collect::<Vec<_>>()
                }) {
                    let command = req.remove(0);
                    let args = req;
                    Standard::new(command, args).into()
                } else {
                    todo!()
                }
            }
            Err(value) => {
                if let Ok(s) = value.into_string() {
                    tracing::warn!("got request from single (`SimpleString` or `BulkString`) instead of `Array`: [{s}]");
                    Standard::new_empty(s).into()
                } else {
                    todo!()
                }
            }
        }
    }
}

impl From<Request> for resp::Value {
    fn from(value: Request) -> Self {
        match value {
            Request::Standard(s) => s.into(),
        }
    }
}

impl From<Standard> for resp::Value {
    fn from(mut value: Standard) -> Self {
        value.args.insert(0, value.command);
        value
            .args
            .into_iter()
            .map(resp::Value::BulkString)
            .collect()
    }
}
