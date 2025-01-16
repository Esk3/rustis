use crate::resp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Request {
    Standard(Standard),
    StandardByteString(StandrardByteString),
}

impl Request {
    #[must_use]
    pub fn command(&self) -> Option<&str> {
        match self {
            Request::Standard(s) => Some(s.command.as_str()),
            Request::StandardByteString(b) => Some(&b.command),
        }
    }

    pub fn into_standard(self) -> Result<Standard, Self> {
        match self {
            Self::Standard(s) => Ok(s),
            _ => Err(self),
        }
    }
    pub fn into_standard_binary(self) -> Result<StandrardByteString, Self> {
        if let Self::StandardByteString(b) = self {
            Ok(b)
        } else {
            Err(self)
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StandrardByteString {
    pub command: String,
    pub args: Vec<Vec<u8>>,
}

impl StandrardByteString {
    #[must_use]
    pub fn new(command: String, args: Vec<Vec<u8>>) -> Self {
        Self { command, args }
    }
}

impl From<StandrardByteString> for Request {
    fn from(value: StandrardByteString) -> Self {
        Self::StandardByteString(value)
    }
}

impl From<super::Message<resp::Value>> for Request {
    fn from(value: super::Message<resp::Value>) -> Self {
        match value.content.into_array() {
            Ok(arr) => {
                if arr.iter().all(resp::value::Value::is_string) {
                    let mut req = arr
                        .into_iter()
                        .map(|v| v.expect_string().unwrap())
                        .collect::<Vec<_>>();
                    let command = req.remove(0);
                    let args = req;
                    Standard::new(command, args).into()
                } else if arr.first().unwrap().is_string()
                    && arr
                        .iter()
                        .skip(1)
                        .all(resp::value::Value::is_into_byte_string)
                {
                    let mut iter = arr.into_iter();
                    let cmd = iter.next().unwrap().expect_string().unwrap();
                    let args = iter
                        .map(resp::value::Value::into_byte_string)
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap();
                    StandrardByteString::new(cmd, args).into()
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
            Request::StandardByteString(_) => todo!(),
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
