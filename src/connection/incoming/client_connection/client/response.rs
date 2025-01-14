use crate::{event, resp};
#[derive(Debug, PartialEq, Eq)]
pub enum ResponseKind {
    Value(resp::Value),
    RecivedReplconf(crate::Request),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub kind: ResponseKind,
    pub event: Option<Vec<event::Kind>>,
}

impl Response {
    #[must_use]
    pub fn new(kind: ResponseKind, event: Option<Vec<event::Kind>>) -> Self {
        Self { kind, event }
    }
    #[must_use]
    pub fn value(value: resp::Value) -> Self {
        Self::new(ResponseKind::Value(value), None)
    }
    #[must_use]
    pub fn value_event(value: resp::Value, event: event::Kind) -> Self {
        Self::new(ResponseKind::Value(value), Some(vec![event]))
    }
    #[must_use]
    pub fn value_events(value: resp::Value, event: Vec<event::Kind>) -> Self {
        Self::new(ResponseKind::Value(value), Some(event))
    }
    #[must_use]
    pub fn ok() -> Self {
        Self::value(resp::Value::ok())
    }
    pub fn into_output(self) -> Result<resp::Value, Self> {
        if let ResponseKind::Value(output) = self.kind {
            Ok(output)
        } else {
            Err(self)
        }
    }
}

impl From<resp::Value> for Response {
    fn from(value: resp::Value) -> Self {
        Self::value(value)
    }
}

impl From<(resp::Value, Option<event::Kind>)> for Response {
    fn from((value, event): (resp::Value, Option<event::Kind>)) -> Self {
        if let Some(event) = event {
            Self::value_event(value, event)
        } else {
            Self::value(value)
        }
    }
}
