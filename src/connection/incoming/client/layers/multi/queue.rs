use crate::{connection::incoming::client, resp::Input};

pub struct Queue(Option<Vec<client::Request>>);

impl Queue {
    pub fn new() -> Self {
        Self(None)
    }
    pub fn is_active(&self) -> bool {
        self.0.is_some()
    }
    pub fn store(&mut self, message: client::Request) -> StoreResult {
        match &mut self.0 {
            None if matches!(message.input, Input::Multi) => {
                self.0 = Some(Vec::new());
                StoreResult::Ok
            }
            Some(_) if matches!(message.input, Input::Multi) => StoreResult::InvalidStore(message),
            None => StoreResult::InvalidStore(message),
            Some(_) if matches!(message.input, Input::CommitMulti) => {
                StoreResult::QueueFinished(self.0.take().unwrap())
            }
            Some(list) => {
                list.push(message);
                StoreResult::Ok
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StoreResult {
    Ok,
    InvalidStore(client::Request),
    QueueFinished(Vec<client::Request>),
}
