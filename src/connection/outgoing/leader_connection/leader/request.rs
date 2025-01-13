pub enum Request {
    Standard(Standard),
}

pub struct Standard {
    pub command: String,
    pub args: Vec<String>,
}

impl Standard {
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
}

impl From<Standard> for Request {
    fn from(value: Standard) -> Self {
        Self::Standard(value)
    }
}
