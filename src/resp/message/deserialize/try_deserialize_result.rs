use crate::resp::{Input, Message, Output, Value};

#[derive(Debug)]
pub enum TryDeserializeResult {
    Ok(Message),
    Err(anyhow::Error),
    Ignore(Vec<Value>),
}

impl TryDeserializeResult {
    #[must_use]
    pub fn new(arr: Vec<Value>) -> Self {
        Self::Ignore(arr)
    }
    #[must_use]
    pub fn try_next<F>(self, f: F) -> Self
    where
        F: Fn(Vec<Value>) -> Self,
    {
        match self {
            TryDeserializeResult::Ok(_) | TryDeserializeResult::Err(_) => self,
            TryDeserializeResult::Ignore(arr) => f(arr),
        }
    }
}

impl From<Message> for TryDeserializeResult {
    fn from(value: Message) -> Self {
        Self::Ok(value)
    }
}

impl From<Input> for TryDeserializeResult {
    fn from(value: Input) -> Self {
        Message::from(value).into()
    }
}

impl From<Output> for TryDeserializeResult {
    fn from(value: Output) -> Self {
        Message::from(value).into()
    }
}

impl Eq for TryDeserializeResult {}

impl PartialEq for TryDeserializeResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ok(l0), Self::Ok(r0)) => l0 == r0,
            (Self::Err(_), Self::Err(_)) => true,
            (Self::Ignore(l0), Self::Ignore(r0)) => l0 == r0,
            _ => false,
        }
    }
}
