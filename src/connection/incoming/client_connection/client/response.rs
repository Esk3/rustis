use crate::{event, resp};

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub value: resp::Value,
    pub events: Option<Vec<event::Kind>>,
}

impl Response {
    #[must_use]
    pub fn new(value: resp::Value, events: Option<Vec<event::Kind>>) -> Self {
        Self { value, events }
    }
    #[must_use]
    pub fn value(value: resp::Value) -> Self {
        Self::new(value, None)
    }
    #[must_use]
    pub fn value_event(value: resp::Value, event: event::Kind) -> Self {
        Self::new(value, Some(vec![event]))
    }
    #[must_use]
    pub fn value_events(value: resp::Value, events: Vec<event::Kind>) -> Self {
        Self::new(value, Some(events))
    }
    #[must_use]
    pub fn ok() -> Self {
        Self::value(resp::Value::ok())
    }
    #[must_use]
    pub fn into_value(self) -> resp::Value {
        self.value
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
